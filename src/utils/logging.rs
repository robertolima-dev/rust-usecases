use elasticsearch::http::response::Response;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, prelude::*};

/// Configura o sistema de logs da aplicação
#[allow(dead_code)]
pub fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    let app_name = "rust-usecases";
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,rust_usecases=debug"));

    let formatting_layer = BunyanFormattingLayer::new(app_name.to_string(), std::io::stdout);

    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

/// Configura logs para desenvolvimento
pub fn setup_development_logging() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("debug,rust_usecases=trace"));

    let formatting_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_timer(tracing_subscriber::fmt::time::time());

    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

pub async fn log_elastic_response(resp: Response) {
    let status = resp.status_code();
    let body_text = resp
        .text()
        .await
        .unwrap_or_else(|_| "Erro ao ler body".to_string());
    println!(
        "✅ Elasticsearch indexado: {:?}, body: {}",
        status, body_text
    );
}
