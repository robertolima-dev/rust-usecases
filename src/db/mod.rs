use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn get_db_pool() -> PgPool {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL não definida no .env");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Erro ao conectar no banco de dados")
}
