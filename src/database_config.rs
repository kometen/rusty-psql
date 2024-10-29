mod tests;

use crate::Vault;
use anyhow::Result;
use serde::Serialize;

#[derive(Serialize, Default)]
pub struct DatabaseConfig {
    host: String,
    user: String,
    name: String,
    pwd: String,
    domain: String,
}

impl DatabaseConfig {
    /// Creates a new DatabaseConfig from vault secrets.
    ///
    /// # Arguments
    ///
    /// * `vault` - The vault containing the database secrets
    ///
    /// # Returns
    ///
    /// * `Result<DatabaseConfig>` - The configured database config or an error
    ///
    /// # Errors
    ///
    /// Returns an error if any required field is missing from the vault
    pub fn from_vault(vault: &Vault) -> Result<Self> {
        Ok(Self {
            host: vault.get_required("db-host")?,
            user: vault.get_required("db-user")?,
            name: vault.get_required("db-name")?,
            pwd: vault.get_required("db-pwd")?,
            domain: vault.get_required("db-domain")?,
        })
    }

    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}@{}.{}/{}",
            self.user, self.host, self.domain, self.name
        )
    }

    pub fn db_keys() -> Vec<String> {
        keys()
    }

    pub fn domain(&self) -> String {
        self.domain.clone()
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn password(&self) -> String {
        self.pwd.clone()
    }
}

pub fn keys() -> Vec<String> {
    let fields: Vec<String> = serde_json::Value::Object(
        serde_json::to_value(DatabaseConfig::default())
            .unwrap()
            .as_object()
            .unwrap()
            .clone(),
    )
    .as_object()
    .unwrap()
    .keys()
    .map(|k| format!("db-{}", k))
    .collect();
    fields
}
