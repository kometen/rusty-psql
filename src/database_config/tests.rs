#[cfg(test)]
mod tests {
    use crate::DatabaseConfig;

    #[test]
    fn test_keys_returns_correct_number_of_fields() {
        let keys = DatabaseConfig::db_keys();
        assert_eq!(keys.len(), 5);
    }

    #[test]
    fn test_keys_have_correct_prefix() {
        let keys = DatabaseConfig::db_keys();
        for key in keys {
            assert!(key.starts_with("db-"));
        }
    }

    #[test]
    fn test_connection_string_format() {
        let config = DatabaseConfig {
            host: "myhost".to_string(),
            user: "myuser".to_string(),
            name: "mydb".to_string(),
            pwd: "mypass".to_string(),
            domain: "mydomain".to_string(),
        };

        assert_eq!(
            config.connection_string(),
            "postgres://myuser@myhost.mydomain/mydb"
        );
    }
}
