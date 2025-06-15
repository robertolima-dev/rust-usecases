use actix::{Actor, Addr};
use actix_web::{App, HttpServer, web};
use anyhow::Result;
use dotenvy::dotenv;
use tracing::info;
use tracing_actix_web::TracingLogger;

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
mod websocket;

#[macro_use]
pub mod macros;

use crate::config::get_settings;
use crate::db::mongo::init_mongodb;
use crate::db::postgres::get_db_pool;
use crate::websocket::server::WsServer;
use elasticsearch::Elasticsearch;
use elasticsearch::http::transport::Transport;
use routes::configure::api_v1_scope;

pub struct AppState {
    pub ws_server: Addr<WsServer>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    config::init_settings().expect("Falha ao inicializar configurações");
    utils::setup_development_logging().expect("Falha ao configurar logs");

    let settings = get_settings();
    let pool = get_db_pool().await;
    let mongo_db = init_mongodb().await.unwrap();
    let transport = Transport::single_node(&settings.elasticsearch.url)?;
    let elastic_client = Elasticsearch::new(transport);

    let local = tokio::task::LocalSet::new();

    local
        .run_until(async move {
            let ws_server = WsServer::new().start();

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
                    .app_data(web::Data::new(AppState {
                        ws_server: ws_server.clone(),
                    }))
                    .service(api_v1_scope())
            })
            .bind((settings.server.host.clone(), settings.server.port))?
            .run()
            .await
        })
        .await?;

    Ok(())
}
