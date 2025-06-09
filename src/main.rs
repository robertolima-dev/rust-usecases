use actix_web::{App, HttpServer, web};
mod db;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

use db::get_db_pool;
// use routes::configure::config;
use routes::configure::api_v1_scope;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let pool = get_db_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // .configure(config)
            .service(api_v1_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
