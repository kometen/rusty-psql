mod tests;

macro_rules! db_config_from_vault {
    ($($field:ident),*) => {
        use crate::Vault;
        use anyhow::Result;
        use serde::Serialize;

        #[derive(Serialize, Default)]
        pub struct DatabaseConfig {
            $(pub $field: String,)*
        }

        impl DatabaseConfig {
            /// Creates a new DatabaseConfig from vault secrets.
            ///
            /// The struct is defined using a macro.
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
                    $($field: vault.get_required(&format!("db-{}", stringify!($field)))?,)*
                })
            }

            pub fn connection_string(&self) -> String {
                format!(
                    "postgres://{}@{}.{}/{}",
                    self.user, self.host, self.domain, self.name
                )
            }

            pub fn db_keys() -> Vec<String> {
                vec![$(format!("db-{}", stringify!($field))),*]
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
    };
}

db_config_from_vault!(host, user, name, pwd, domain);
