// use actix_web::{web,  HttpResponse};
use sqlx::types::chrono::Utc;
// use sqlx::mysql::MySqlPool;
#[derive(sqlx::FromRow, serde::Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: Option<chrono::DateTime<Utc>>,
}

// async fn index(pool: web::Data<MySqlPool>) -> HttpResponse {
//     // 使用数据库连接池执行查询
//     let result = sqlx::query_as::<_, User>("SELECT id, username, created_at FROM users")
//         .fetch_all(pool.get_ref())
//         .await;

//     match result {
//         Ok(users) => HttpResponse::Ok().json(users),
//         Err(e) => {
//             eprintln!("Failed to execute query: {:?}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }

