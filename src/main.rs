use actix_web::{App, HttpServer, web};
mod db;
mod errors;
mod extensions;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;
mod config;

use db::get_db_pool;
// use routes::configure::config;
use routes::configure::api_v1_scope;
use config::init_settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    
    // Inicializa as configurações
    init_settings().expect("Falha ao inicializar configurações");
    
    // Obtém as configurações
    let settings = config::get_settings();
    let pool = get_db_pool().await;

    println!("🚀 Iniciando servidor em {}:{}", settings.server.host, settings.server.port);
    println!("🌍 Ambiente: {:?}", settings.environment);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // .configure(config)
            .service(api_v1_scope())
    })
    .bind((settings.server.host, settings.server.port))?
    .run()
    .await
}
