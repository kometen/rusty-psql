#[cfg(test)]
mod tests {
    use crate::{DatabaseConfig, VaultStorage};
    use anyhow::Result;
    use std::collections::HashMap;

    // Mock vault for testing
    struct MockVault {
        secrets: HashMap<String, String>,
    }

    impl MockVault {
        fn new() -> Self {
            let mut secrets = HashMap::new();
            secrets.insert("db-host".to_string(), "testhost".to_string());
            secrets.insert("db-user".to_string(), "testuser".to_string());
            secrets.insert("db-name".to_string(), "testdb".to_string());
            secrets.insert("db-pwd".to_string(), "testpass".to_string());
            secrets.insert("db-domain".to_string(), "testdomain".to_string());
            Self { secrets }
        }
    }

    impl VaultStorage for MockVault {
        fn get_required(&self, key: &str) -> Result<String> {
            self.secrets
                .get(key)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Required key '{}' not found", key))
        }
    }

    #[test]
    fn test_database_config_from_vault() {
        let mock_vault = MockVault::new();
        let config = DatabaseConfig::from_vault(&mock_vault).unwrap();

        assert_eq!(config.host, "testhost");
        assert_eq!(config.user, "testuser");
        // ... etc
    }

    #[test]
    fn test_missing_key_returns_error() {
        let mut mock_vault = MockVault::new();
        mock_vault.secrets.remove("db-host");

        let result = DatabaseConfig::from_vault(&mock_vault);
        assert!(result.is_err());
    }
}
