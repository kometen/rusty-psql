pub mod connect {
    use std::io::Write;
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
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(format!("{}\n", &vault.pwd).as_bytes())?;
        }

        let status = child.wait()?;

        if !status.success() {
            return Err("psql command failed".into());
        }

        Ok(())
    }
}
