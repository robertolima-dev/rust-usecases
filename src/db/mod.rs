use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use crate::config::get_settings;

pub async fn get_db_pool() -> PgPool {
    dotenv().ok();

    let settings = get_settings();
    
    PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .connect(&settings.database.url)
        .await
        .expect("Erro ao conectar no banco de dados")
}
