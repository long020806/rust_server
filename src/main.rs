use actix_web::{web, App, HttpServer};
mod hello;
use hello::hello;
mod my_repsonse;
use sqlx::mysql::MySqlPool;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
   // 配置数据库连接池
   let database_url = "mysql://dragon:dragon@localhost:3306/rust"; // 使用 SQLite 数据库，你可以更改为其他数据库的连接字符串
   let pool = MySqlPool::connect(database_url).await.unwrap();

   // 运行数据库迁移（如果需要的话）
   sqlx::migrate!("./migrations")
       .run(&pool)
       .await
       .expect("Failed to run migrations");

    HttpServer::new(move|| {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/").to(hello))
            .service(web::resource("/test").to(hello::get_data))
            .service(hello::test2)
            .service(hello::get_data_mysql)
            .service(hello::add_data_mysql)
            .service(hello::rand_data_mysql)
            .service(hello::json_data_mysql)
            .service(hello::rand2_data_mysql)
            .service(hello::rand3_data_mysql)
            .service(hello::detail)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
