use clap::{Parser, Subcommand};
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPool;
use std::env;
use std::path::Path;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Migrate,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Hello, world!");

    // Load .env file
    dotenvy::dotenv()?;

    let args = Args::parse();
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    if pool.is_closed() {
        tracing::error!("Failed to connect to database");
        return Ok(());
    }

    match args.cmd {
        Some(Command::Migrate) => {
            migrate(&pool).await?
        }
        None => {
            tracing::info!("No command provided")
        }
    }

    Ok(())
}

async fn migrate(pool: &PgPool) -> anyhow::Result<()> {
    tracing::info!("Migrations running...");

    // Read migrations from a local folder: ./ migrations
    let m = Migrator::new(Path::new("./migrations")).await?;

    // Run the migrations
    m.run(pool).await?;

    tracing::info!("Migrations ran successfully!");
    Ok(())
}