//! Secret Management Module
//!
//! This module manages secrets, retrieving Azure Key Vault URL
//! from 1password via the command line utility `op`.
//!

mod tests;

use anyhow::{Context, Result};
use std::process::Command;

/// URL of the Azure Key Vault.
pub struct SecretManager {
    pub url: String,
}

impl SecretManager {
    /// Creates a new Secret Manager instance with a specific value.
    ///
    /// # Arguments
    ///
    /// * `key` - Name of the variable with the value of the 1password path
    ///
    /// # Returns
    ///
    /// A Result containing the SecretManager if successful, or an error if the secret
    /// could not be retrieved.
    ///
    /// # Example
    ///
    /// ```
    /// use rusty_psql::SecretManager;
    /// use anyhow::Result;
    ///
    /// fn example() -> Result<()> {
    ///     let secret_manager = SecretManager::new("AZURE_KEY_VAULT_TEST")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(key: &str) -> Result<Self> {
        let op_path = format!("op://Production/AzureKeyVault{}/credentials/url", key);

        let command = Command::new("op")
            .arg("read")
            .arg(&op_path)
            .output()
            .context("Error executing command")?;

        let url = String::from_utf8(command.stdout)
            .context("Failed to convert command output to string")?
            .trim_end()
            .to_string();

        Ok(Self { url })
    }

    /// Used for testing an error is returned if the command line utility
    /// is not present.
    #[cfg(test)]
    fn wrong_command_for_test() -> Result<Self> {
        let command = Command::new("_op_")
            .arg("read")
            .arg("foo")
            .output()
            .context("Error executing command")?;

        let url = String::from_utf8(command.stdout)
            .context("Failed to convert command output to string")?
            .trim_end()
            .to_string();

        Ok(Self { url })
    }
}
