
use actix_web::{HttpResponse, http::StatusCode, get,post,web};
use chrono::Utc;
use rand::{distributions::Alphanumeric, prelude::Distribution};
use serde::{Serialize, Deserialize};
use crate::my_repsonse as response;
mod hello;
use hello::User;
use sqlx::{mysql::MySqlPool, MySqlConnection};
mod data;
use data::MyJsonData;
#[derive(Debug, Serialize)]
struct MyData {
    key: String,
    value: i32,
}

#[get("/test2")]
async fn test2() -> HttpResponse{
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
pub async fn get_data_mysql(pool: web::Data<MySqlPool>) -> HttpResponse{
        // 使用数据库连接池执行查询
        let result = sqlx::query_as::<_, User>("SELECT id, username, created_at FROM users limit 0,10")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(users) => response::json_response(StatusCode::OK, "Data retrieved successfully", Some(users)),
        Err(e) => {
            eprintln!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/mysql/data/add")]
pub async fn add_data_mysql(pool: web::Data<MySqlPool>,username : String) -> HttpResponse{


        // 使用数据库连接池执行查询
        let result = sqlx::query!("INSERT INTO users (username) VALUES (?)", username)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => response::json_response(StatusCode::OK, "Data retrieved successfully",Some(())),
        Err(e) => {
            eprintln!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
#[derive(Deserialize,Debug)]
struct MyQuery {
    value: i32,
}


#[post("/mysql/data/rand")]
pub async fn rand_data_mysql(pool: web::Data<MySqlPool>,data:web::Query<MyQuery>) -> HttpResponse{
    let mut rng = rand::thread_rng();
    let mut result:Vec<String> = vec![];
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
            eprintln!(" start insert start {}",Utc::now());
            for username in result {
                let _ = sqlx::query!(
                    "INSERT INTO users (username) VALUES (?)",
                    username
                )
                .execute(&mut tx as &mut MySqlConnection)
                .await;
            }
            eprintln!(" end insert end {}",Utc::now());
            eprintln!(" start insert commit start  {}",Utc::now());
            let commit = tx.commit().await;
            eprintln!(" start insert commit  end {}",Utc::now());
            match commit {
                Ok(_)=>{
                    response::json_response(StatusCode::OK, "Data retrieved successfully",Some(()))
                },
                Err(e)=>{
                    eprintln!("Failed to execute query: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/mysql/data/rand2")]
pub async fn rand2_data_mysql(pool: web::Data<MySqlPool>,data:web::Query<MyQuery>) -> HttpResponse{
    //  10w 数据 1342ms   事务提交 17.31s
    let mut rng = rand::thread_rng();
    let mut result:Vec<String> = vec![];
    for _ in 0..data.value {
        let s: String = Alphanumeric
        .sample_iter(&mut rng)
        .take(7)
        .map(char::from)
        .collect::<String>()
        .to_uppercase();
        result.push("('".to_owned()+&s+"')")
    }
    // let params = vec!["test1","test2"];
    // let sql = "INSERT INTO users (username) VALUES (?)";
    let sql = "INSERT INTO users (username) VALUES ".to_owned() + &result.join(",");
    match sqlx::query(&sql).execute(pool.get_ref()).await {
        Ok(_) => {
            response::json_response(StatusCode::OK, "Data retrieved successfully", Some(sql))
        },
        Err(e) => {
            eprintln!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}





#[post("/mysql/data/json")]
pub async fn json_data_mysql(data:web::Json<MyJsonData>,pool: web::Data<MySqlPool>) -> HttpResponse{
 // 使用数据库连接池执行查询
    let offset = (data.page - 1) *data.size;
    let limit = data.size;
    let result:Result<Vec<User>,_> = sqlx::query_as!(User,"SELECT id, username, created_at FROM users limit ?,?",offset,limit)
    .fetch_all(pool.get_ref())
    .await;

    match result {
    Ok(users) => response::json_response(StatusCode::OK, "Data retrieved successfully", Some(users)),
    Err(e) => {
        eprintln!("Failed to execute query: {:?}", e);
        HttpResponse::InternalServerError().finish()
        }
    }

}
