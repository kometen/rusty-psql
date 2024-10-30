pub mod connect {
    use crate::{DatabaseConfig, VaultStorage};
    use anyhow::Result;
    use std::process::{Command, Stdio};

    /// Performs the login to the database.
    pub fn run_psql(vault: &impl VaultStorage) -> Result<(), Box<dyn std::error::Error>> {
        let config = DatabaseConfig::from_vault(vault)?;

        let mut child = Command::new("psql")
            .arg(&config.connection_string())
            .env("PGPASSWORD", &config.password())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        let status = child.wait()?;

        if !status.success() {
            return Err("psql command failed".into());
        }

        Ok(())
    }
}
