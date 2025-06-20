use dotenvy::dotenv;
use rust_usecases::db::postgres::get_db_pool;
use sqlx::PgPool;
use tracing::info;

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Carregar .env
    dotenv().ok();

    // Inicializar settings (necessário se seu pool usa o settings)
    rust_usecases::config::init_settings().expect("Falha ao inicializar configurações");

    // Opcional: Inicializar logs
    rust_usecases::utils::setup_development_logging().expect("Falha ao configurar logs");

    info!("🚀 Executando migrations via CLI...");

    let pool: PgPool = get_db_pool().await;

    // Executa todas as migrations pendentes
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => {
            info!("✅ Migrations rodadas com sucesso!");
            println!("✅ Migrations concluídas com sucesso!");
        }
        Err(err) => {
            eprintln!("❌ Erro ao rodar migrations: {:?}", err);
            return Err(Box::new(err));
        }
    }

    Ok(())
}
