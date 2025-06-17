use crate::websocket::server::WsServer;
use actix::Addr;
use elasticsearch::Elasticsearch;
use mongodb::Database;
use sqlx::PgPool;

pub struct AppState {
    pub db: PgPool,
    pub mongo: Database,
    pub es: Elasticsearch,
    pub ws_server: Addr<WsServer>,
}
