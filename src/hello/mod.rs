



use crate::{my_repsonse as response, hello::data::{MyQuery, MyDetailQuery, TestJsonData}};
use actix_web::{get, http::StatusCode, post, web, HttpResponse};
use chrono::Utc;
use rand::{distributions::Alphanumeric, prelude::Distribution};

mod hello;
use hello::User;
use regex::Regex;
use sqlx::{mysql::MySqlPool, MySqlConnection};
mod data;
use data::MyJsonData;

use self::data::{MyData, UserVo};

#[get("/test2")]
async fn test2() -> HttpResponse {
    HttpResponse::Ok().body("test2")
}

pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().json("Hello, world!")
}

pub async fn get_data() -> HttpResponse {
    // 模拟一些数据
    let data = MyData {
        key: String::from("example"),
        value: 42,
    };

    // 返回封装的 JSON 响应
    response::json_response(StatusCode::OK, "Data retrieved successfully", Some(data))
}
#[post("/mysql/data/get")]
pub async fn get_data_mysql(pool: web::Data<MySqlPool>) -> HttpResponse {
    // 使用数据库连接池执行查询
    let result = sqlx::query_as::<_, User>("SELECT id, username, created_at FROM users limit 0,10")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(users) => {
            response::json_response(StatusCode::OK, "Data retrieved successfully", Some(users))
        }
        Err(e) => {
            eprintln!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/mysql/data/add")]
pub async fn add_data_mysql(pool: web::Data<MySqlPool>, username: String) -> HttpResponse {
    // 使用数据库连接池执行查询
    let result = sqlx::query!("INSERT INTO users (username) VALUES (?)", username)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => response::json_response(StatusCode::OK, "Data retrieved successfully", Some(())),
        Err(e) => {
            eprintln!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/mysql/data/rand")]
pub async fn rand_data_mysql(
    pool: web::Data<MySqlPool>,
    data: web::Query<MyQuery>,
) -> HttpResponse {
    let mut rng = rand::thread_rng();
    let mut result: Vec<String> = vec![];
    for _ in 0..data.value {
        let s: String = Alphanumeric
            .sample_iter(&mut rng)
            .take(7)
            .map(char::from)
            .collect::<String>()
            .to_uppercase();
        result.push(s)
    }
    let tx = pool.begin().await;
    match tx {
        Ok(mut tx) => {
            eprintln!(" start insert start {}", Utc::now());
            for username in result {
                let _ = sqlx::query!("INSERT INTO users (username) VALUES (?)", username)
                    .execute(&mut tx as &mut MySqlConnection)
                    .await;
            }
            eprintln!(" end insert end {}", Utc::now());
            eprintln!(" start insert commit start  {}", Utc::now());
            let commit = tx.commit().await;
            eprintln!(" start insert commit  end {}", Utc::now());
            match commit {
                Ok(_) => {
                    response::json_response(StatusCode::OK, "Data retrieved successfully", Some(()))
                }
                Err(e) => {
                    eprintln!("Failed to execute query: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/mysql/data/rand2")]
pub async fn rand2_data_mysql(
    pool: web::Data<MySqlPool>,
    data: web::Query<MyQuery>,
) -> HttpResponse {
    if data.value >= 2_000_000 {
        response::json_response(StatusCode::NOT_FOUND, "参数过大,请小于200w", Some(()))
    } else {
        //  10w 数据 1342ms   事务提交 17.31s
        let mut rng = rand::thread_rng();
        let mut result: Vec<String> = vec![];
        for _ in 0..data.value {
            let s: String = Alphanumeric
                .sample_iter(&mut rng)
                .take(7)
                .map(char::from)
                .collect::<String>()
                .to_uppercase();
            result.push("('".to_owned() + &s + "')")
        }
        // let params = vec!["test1","test2"];
        // let sql = "INSERT INTO users (username) VALUES (?)";
        let sql = "INSERT INTO users (username) VALUES ".to_owned() + &result.join(",");
        match sqlx::query(&sql).execute(pool.get_ref()).await {
            Ok(_) => {
                response::json_response(StatusCode::OK, "Data retrieved successfully", Some(()))
            }
            Err(e) => {
                eprintln!("Failed to execute query: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

#[post("/mysql/data/rand3")]
pub async fn rand3_data_mysql(
    _pool: web::Data<MySqlPool>,
    _data: web::Query<MyQuery>,
) -> HttpResponse {
    response::json_response(StatusCode::OK, "Data retrieved successfully", Some(()))
}

#[post("/mysql/data/json")]
pub async fn json_data_mysql(
    data: web::Json<MyJsonData>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    // 使用数据库连接池执行查询
    let mut offset = (data.page - 1) * data.size;
    let limit = data.size;
    eprintln!("count start :{}", Utc::now());
    let count_result: Result<i32, sqlx::Error> = sqlx::query_scalar("select count(1) from users")
        .fetch_one(pool.get_ref())
        .await;
    eprintln!("count end :{}", Utc::now());
    match count_result {
        Ok(count) => {
            let mut page = data.page;
            let mut pages = count / data.size;
            if page < 1 {
                page = 1;
            }
            if count > data.size * pages {
                pages = pages + 1;
            }
            if offset > count {
                page = pages;
            }
            offset = (page - 1) * data.size;
            eprintln!("data start :{}", Utc::now());
            let result: Result<Vec<User>, _> = sqlx::query_as!(
                User,
                "SELECT id, username, created_at FROM users order by id limit ?,?",
                offset,
                limit
            )
            .fetch_all(pool.get_ref())
            .await;
            eprintln!("data end :{}", Utc::now());
            match result {
                Ok(users) => {
                    let user_vo:Vec<UserVo> = users.iter().map(|item|{
                        trans_user_vo(item)
                    }).collect();
                    response::json_page_response(
                        StatusCode::OK,
                        "Data retrieved successfully",
                        Some(user_vo),
                        page,
                        pages,
                        count,
                    )
                },
                Err(e) => {
                    eprintln!("Failed to execute query: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/mysql/data/detail")]
pub async fn detail(pool: web::Data<MySqlPool>, data: web::Query<MyDetailQuery>) -> HttpResponse {
    eprintln!("select start time:{}",Utc::now());
    let query_result = sqlx::query_as!(
        User,
        "select id, username, created_at from users where id = ?",
        data.id
    )
    .fetch_one(pool.get_ref())
    .await;
    eprintln!("select end time:{}",Utc::now());

    match query_result {
        Ok(user) => {
            response::json_response(StatusCode::OK, "Data retrieved successfully", Some(user))
        }
        Err(e) => {
            eprintln!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


fn trans_user_vo(user:&User)->UserVo{
    UserVo { id: user.id, username: user.username.clone(), created_at: match user.created_at {
        Some(date) => {
           Some(date.format("%Y-%m-%d %H:%M").to_string())
        }
        None => Option::None,
    } }
}


#[post("/mysql/data/json2")]
pub async fn json2_data_mysql(
    _data: web::Json<MyJsonData>,
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
   let page_user = test1(_data.page>100);
    match page_user {
        Ok(users) => {
            response::json_response(StatusCode::OK, "Data retrieved successfully", Some(users))
        },
        Err(e)=>{
            response::json_response(StatusCode::BAD_REQUEST, e, Some(()))
        }
    }
}

fn test1(bool:bool)->Result<Vec<User>, &'static str>{
    if bool {
        return Result::Ok(vec![])
    }else{
        return Result::Err("发生错误");
    }
}


#[post("/mysql/data/test")]
pub async fn test(
    _data: web::Json<TestJsonData>,
    _pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let temp = test_regex(_data.value.clone());
    response::json_response(StatusCode::OK, "成功", Some(temp))
}

fn test_regex(str:String) -> String  {
    let reg = match Regex::new(r"\B(?=(\d{3})+(?!\d))") {
        Ok(reg) => {
            let replace_txt = reg.replace_all(str.as_str(), ",");
            replace_txt.into_owned()
        },
        Err(_) => {
            let result = "正则不支持".to_string();
            result
        },
    };
    reg

}