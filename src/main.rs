use actix_web::{App, HttpServer, web};
mod config;
mod db;
mod errors;
mod extensions;
mod logs;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;
#[macro_use]
pub mod macros;

use crate::config::get_settings;
use anyhow::Result;
use config::init_settings;
use db::mongo::init_mongodb;
use db::postgres::get_db_pool;
use elasticsearch::Elasticsearch;
use elasticsearch::http::transport::Transport;
use routes::configure::api_v1_scope;
use tracing::info;
use tracing_actix_web::TracingLogger;

// #[actix_web::main]
#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    // Inicializa as configurações
    init_settings().expect("Falha ao inicializar configurações");

    // Configura o sistema de logs
    utils::setup_development_logging().expect("Falha ao configurar logs");

    // Obtém as configurações
    let settings = get_settings();
    let pool = get_db_pool().await;
    let mongo_db = init_mongodb().await.unwrap();

    let transport = Transport::single_node(&settings.elasticsearch.url)?;
    let elastic_client = Elasticsearch::new(transport);

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
            .app_data(web::Data::new(mongo_db.clone()))
            .app_data(web::Data::new(elastic_client.clone()))
            .service(api_v1_scope())
    })
    .bind((settings.server.host, settings.server.port))?
    .run()
    .await?;

    Ok(())
}
