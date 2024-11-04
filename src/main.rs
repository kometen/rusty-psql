use anyhow::Result;
use azure_vault_secrets::Vault;
use clap::Parser;
use rusty_psql::DatabaseConfig;
use rusty_psql::{check_dns, run_psql};
use secret_manager_1password::SecretManager;

// Command line arguments with clap.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    namespace: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let secret_manager = SecretManager::new(cli.namespace.as_str())?;
    let db_keys = DatabaseConfig::db_keys();
    let vault = Vault::new(secret_manager.url.as_str(), db_keys).await?;

    if let Err(e) = check_dns(&vault).await {
        eprintln!("DNS resolution failed: {}", e);
        eprintln!("Root cause: {}", e.root_cause());
        return Err(e);
    }

    if let Err(err) = run_psql(&vault) {
        eprintln!("Error connecting to database: {:#}", err);
        std::process::exit(1);
    }

    Ok(())
}
