use actix_web::{App, HttpServer, web};
mod config;
mod db;
mod errors;
mod extensions;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

use db::get_db_pool;
// use routes::configure::config;
use config::init_settings;
use routes::configure::api_v1_scope;
use tracing::info;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    // Inicializa as configurações
    init_settings().expect("Falha ao inicializar configurações");

    // Configura o sistema de logs
    utils::setup_development_logging().expect("Falha ao configurar logs");

    // Obtém as configurações
    let settings = config::get_settings();
    let pool = get_db_pool().await;

    info!(
        host = %settings.server.host,
        port = %settings.server.port,
        environment = ?settings.environment,
        "🚀 Iniciando servidor"
    );

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(pool.clone()))
            // .configure(config)
            .service(api_v1_scope())
    })
    .bind((settings.server.host, settings.server.port))?
    .run()
    .await
}
