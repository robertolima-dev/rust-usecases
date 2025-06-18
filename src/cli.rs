use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Rust Usecases CLI")]
#[command(version = "1.0")]
#[command(about = "Ferramentas administrativas da aplicação", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Iniciar o servidor HTTP (API)
    Api,
    /// Sincroniza cursos com Elasticsearch
    SyncCourses,
    /// Rodar migrations
    Migrate,
    /// Rodar seeds
    Seed,
}
