use crate::config::get_settings;
use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::{error, info};

pub async fn get_db_pool() -> PgPool {
    dotenv().ok();

    let settings = get_settings();

    info!(
        max_connections = %settings.database.max_connections,
        "Conectando ao banco de dados"
    );

    match PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .connect(&settings.database.url)
        .await
    {
        Ok(pool) => {
            info!("Conexão com o banco de dados estabelecida com sucesso");
            pool
        }
        Err(err) => {
            error!(error = %err, "Falha ao conectar ao banco de dados");
            panic!("Erro ao conectar no banco de dados: {}", err);
        }
    }
}
