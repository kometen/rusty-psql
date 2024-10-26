pub mod connect {
    use std::process::{Command, Stdio};

    use crate::{Environment, Vault};

    pub fn run_psql(
        vault: &Vault,
        environment: &Environment,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let connection_string = format!(
            "postgres://{}@{}.{}/{}",
            &vault.user, &vault.host, &environment.domain, &vault.name
        );

        let mut child = Command::new("psql")
            .arg(connection_string)
            .env("PGPASSWORD", &vault.pwd)
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
