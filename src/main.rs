mod cli;
mod cli_init;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{migrate, sync_courses};
use dotenvy::dotenv;
use rust_usecases::server::start_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Api => {
            start_server().await?;
        }
        Commands::SyncCourses => {
            sync_courses::run().await?;
        }
        Commands::Migrate => {
            migrate::run().await?;
        }
        Commands::Seed => {
            println!("(TODO) Seed ainda não implementado.");
        }
    }

    Ok(())
}
