use actix::Actor;
use actix_web::{App, HttpServer, web};
use tokio::task::LocalSet;
use tracing::info;
use tracing_actix_web::TracingLogger;

use crate::config::{app_state::AppState, get_settings};
use crate::db::{elasticsearch::get_elastic_client, mongo::init_mongodb, postgres::get_db_pool};
use crate::routes::configure::api_v1_scope;
use crate::utils::setup_development_logging;
use crate::websocket::server::WsServer;

pub async fn start_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🚀 Iniciando Actix Web Server...");

    setup_development_logging().expect("Falha ao configurar logs");

    let settings = get_settings();
    let pool = get_db_pool().await;
    let mongo_db = init_mongodb().await.unwrap();
    let elastic_client = get_elastic_client()?;

    let local = LocalSet::new();

    local
        .run_until(async move {
            let ws_server = WsServer::new().start();
            info!(
                host = %settings.server.host,
                port = %settings.server.port,
                environment = ?settings.environment,
                "🚀 Servidor Actix iniciado"
            );

            let app_state = web::Data::new(AppState {
                db: pool,
                mongo: mongo_db,
                es: elastic_client,
                ws_server,
            });

            HttpServer::new(move || {
                App::new()
                    .wrap(TracingLogger::default())
                    .app_data(app_state.clone())
                    .service(api_v1_scope())
            })
            .bind((settings.server.host.clone(), settings.server.port))?
            .run()
            .await
        })
        .await?;

    Ok(())
}
