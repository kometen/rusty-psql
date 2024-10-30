//! Vault Module
//!
//! This module manages secrets retrieved from Azure Key Vault.
//!

mod tests;

use anyhow::{Context, Result};
use azure_security_keyvault::SecretClient;
use std::collections::HashMap;

pub struct Vault {
    pub secrets: HashMap<String, String>,
}

pub trait VaultStorage {
    fn get_required(&self, key: &str) -> Result<String>;
}

impl Vault {
    /// Creates a Vault instance with Azure Key Vault secrets.
    ///
    /// # Arguments
    ///
    /// * `url` - URL
    ///
    /// # Returns
    ///
    /// A Result containing the Vault secrets if successful, or an error if the secrets
    /// could not be retrieved.
    ///
    /// # Example
    ///
    /// ```
    /// use rusty_psql::Vault;
    /// use anyhow::Result;
    ///
    /// async fn example() -> Result<()> {
    ///     let secret_keys = vec!["".to_string()];
    ///     let vault = Vault::new("AZURE_KEY_VAULT_TEST", secret_keys).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(url: &str, db_keys: Vec<String>) -> Result<Self> {
        let mut secrets = HashMap::new();

        let credential =
            azure_identity::create_credential().context("Failed to create credentials")?;
        let client = SecretClient::new(url, credential)
            .context("Failed to create a SecretClient instance")?;

        let keys_iter = db_keys.iter();
        for key in keys_iter {
            secrets.insert(key.clone(), get_secret(&client, key.clone()).await?);
        }

        Ok(Self { secrets })
    }
}

impl VaultStorage for Vault {
    /// Creates a VaultStorage instance.
    ///
    /// # Arguments
    ///
    /// * `self` - Vault
    /// * `key` - The Azure Key Vault secret
    ///
    /// # Returns
    ///
    /// A Result containing the Vault secrets if successful, or an error if the secrets
    /// could not be retrieved.
    ///
    /// # Example
    ///
    /// ```
    /// use rusty_psql::{Vault, VaultStorage};
    /// use anyhow::Result;
    ///
    /// async fn example() -> Result<()> {
    ///     let secret_keys = vec!["".to_string()];
    ///     let vault = Vault::new("AZURE_KEY_VAULT_TEST", secret_keys).await?;
    ///     let secret_key = VaultStorage::get_required(&vault, "")?;
    ///     Ok(())
    /// }
    /// ```
    fn get_required(&self, key: &str) -> Result<String> {
        self.secrets
            .get(key)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Required key '{}' not found", key))
    }
}

async fn get_secret(client: &SecretClient, key: String) -> Result<String> {
    let response = client.get(key).await.context("Unable to retrieve value")?;
    Ok(response.value.to_string())
}
