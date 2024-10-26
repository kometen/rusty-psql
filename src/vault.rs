//! Vault Module
//!
//! This module manages secrets retrieved from Azure Key Vault.
//!

use anyhow::{Context, Result};
use azure_security_keyvault::SecretClient;

pub struct Vault {
    pub host: String,
    pub user: String,
    pub name: String,
    pub pwd: String,
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
    ///     let vault = Vault::new("AZURE_KEY_VAULT_TEST").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(url: &str) -> Result<Self> {
        let credential =
            azure_identity::create_credential().context("Failed to create credentials")?;
        let client = SecretClient::new(url, credential)
            .context("Failed to create a SecretClient instance")?;

        let host = get_secret(&client, String::from("db-host")).await;
        let user = get_secret(&client, String::from("db-user")).await;
        let name = get_secret(&client, String::from("db-name")).await;
        let pwd = get_secret(&client, String::from("db-pwd")).await;

        Ok(Self {
            host,
            user,
            name,
            pwd,
        })
    }
}

async fn get_secret(client: &SecretClient, key: String) -> String {
    client
        .get(key.clone())
        .await
        .map_err(|e| format!("Error fetching secret using key {}: {}", key, e))
        .unwrap()
        .value
}
