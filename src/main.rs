mod cli;
mod cli_init;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};
use commands::sync_courses;
use dotenvy::dotenv;
use rust_usecases::server::start_server;

// use rust_usecases::config::init_settings;

fn init_cli_env() {
    dotenv().ok();
    rust_usecases::config::init_settings().expect("Falha ao inicializar configurações");
    rust_usecases::utils::setup_development_logging().expect("Falha ao configurar logs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    // init_settings().expect("Falha ao inicializar configurações");

    let cli = Cli::parse();

    match cli.command {
        Commands::Api => {
            start_server().await?;
        }
        Commands::SyncCourses => {
            init_cli_env();
            sync_courses::run().await?;
        }
        Commands::Migrate => {
            println!("(TODO) Migrate ainda não implementado.");
        }
        Commands::Seed => {
            println!("(TODO) Seed ainda não implementado.");
        }
    }

    Ok(())
}

// use actix::Actor;
// use actix_web::{App, HttpServer, web};
// use anyhow::Result;
// use dotenvy::dotenv;
// use tracing::info;
// use tracing_actix_web::TracingLogger;

// mod config;
// mod db;
// mod errors;
// mod extensions;
// mod logs;
// mod middleware;
// mod models;
// mod repositories;
// mod routes;
// mod services;
// mod utils;
// mod websocket;

// #[macro_use]
// pub mod macros;

// use crate::config::app_state::AppState;
// use crate::config::get_settings;
// use crate::db::mongo::init_mongodb;
// use crate::db::postgres::get_db_pool;
// use crate::websocket::server::WsServer;
// use elasticsearch::Elasticsearch;
// use elasticsearch::http::transport::Transport;
// use routes::configure::api_v1_scope;

// #[tokio::main]
// async fn main() -> Result<()> {
//     dotenv().ok();
//     config::init_settings().expect("Falha ao inicializar configurações");
//     utils::setup_development_logging().expect("Falha ao configurar logs");

//     let settings = get_settings();
//     let pool = get_db_pool().await;
//     let mongo_db = init_mongodb().await.unwrap();
//     let transport = Transport::single_node(&settings.elasticsearch.url)?;
//     let elastic_client = Elasticsearch::new(transport);

//     let local = tokio::task::LocalSet::new();

//     local
//         .run_until(async move {
//             let ws_server = WsServer::new().start();

//             info!(
//                 host = %settings.server.host,
//                 port = %settings.server.port,
//                 environment = ?settings.environment,
//                 "🚀 Iniciando servidor"
//             );

//             let app_state = web::Data::new(AppState {
//                 db: pool,
//                 mongo: mongo_db,
//                 es: elastic_client,
//                 ws_server: ws_server,
//             });

//             HttpServer::new(move || {
//                 App::new()
//                     .wrap(TracingLogger::default())
//                     .app_data(app_state.clone())
//                     .service(api_v1_scope())
//             })
//             .bind((settings.server.host.clone(), settings.server.port))?
//             .run()
//             .await
//         })
//         .await?;

//     Ok(())
// }
