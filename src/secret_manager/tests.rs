#[cfg(test)]
mod tests {
    use super::super::SecretManager;
    use std::env;

    /// Tests SecretManager creation with a valid environment variable.
    ///
    /// This handles two scenarios:
    /// 1. In Github Actions: Connects to 1password.com with a token stored
    /// as a secret in settings on the github-repository and fetches the
    /// Azure Key Vault URL from 1password.com.
    ///
    /// 2. Locally: Uses the `op` command to retrieve the Azure Key Vault URL.
    #[test]
    fn test_new_with_valid_env_var() {
        if env::var("GITHUB_ACTIONS").is_ok() {
            let result = env::var("AZURE_KEY_VAULT_TEST");
            assert!(result.is_ok());
            let secret_manager = result.unwrap();
            assert_eq!(secret_manager, "https://foo.bar.baz.net/");
        } else {
            let result = SecretManager::new("test");
            assert!(result.is_ok());
            let secret_manager = result.unwrap();
            assert_eq!(secret_manager.url, "https://foo.bar.baz.net/");
        }
    }

    /// Test an error is returned if the command line utility `op` is not
    /// present.
    #[test]
    fn test_new_with_invalid_command() {
        let result = SecretManager::wrong_command_for_test();
        assert!(result.is_err());
    }
}
