use dotenvy::dotenv;
use rust_usecases::config;
use tracing::info;

pub fn init_cli_environment() {
    dotenv().ok();

    if let Err(err) = config::init_settings() {
        eprintln!("❌ Falha ao inicializar configurações: {:?}", err);
        std::process::exit(1);
    }

    if let Err(err) = rust_usecases::utils::setup_development_logging() {
        eprintln!("❌ Falha ao configurar logs: {:?}", err);
        std::process::exit(1);
    }

    info!("✅ Ambiente CLI inicializado com sucesso!");
}
