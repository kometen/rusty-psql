use anyhow::Result;
use clap::Parser;
use rusty_psql::{check_dns, run_psql, Environment, SecretManager, Vault};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    namespace: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let environment = Environment::new()?;
    let secret_manager = SecretManager::new(cli.namespace.as_str())?;
    let vault = Vault::new(secret_manager.url.as_str()).await?;
    if let Err(e) = check_dns(&vault, &environment).await {
        //eprintln!("DNS resolution failed: {}", e);
        //eprintln!("Root cause: {}", e.root_cause());
        return Err(e);
    }

    let _ = run_psql(&vault, &environment);

    Ok(())
}
