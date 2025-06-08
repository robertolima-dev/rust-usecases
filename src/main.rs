use actix_web::{App, HttpServer, web};
mod db;
mod models;
mod routes;
mod services;

use db::get_db_pool;
use routes::configure::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_db_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
