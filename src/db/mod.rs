use crate::config::get_settings;
use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn get_db_pool() -> PgPool {
    dotenv().ok();

    let settings = get_settings();

    PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .connect(&settings.database.url)
        .await
        .expect("Erro ao conectar no banco de dados")
}
