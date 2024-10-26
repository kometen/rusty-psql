#[cfg(test)]
mod tests {
    use crate::Environment;

    /// Test Environment creation with a valid domain.
    ///
    /// Validate a missing domain throws an error.
    #[test]
    fn test_error_is_returned_when_domain_is_not_set() {
        let env_1 = Environment::with_domain(None);
        assert!(env_1.is_err());
    }

    /// Validate a domain is set.
    #[test]
    fn test_ok_is_returned_when_domain_is_set() {
        let env_2 = Environment::with_domain(Some("foo.bar.baz".to_string()));
        assert_eq!(env_2.unwrap().domain, "foo.bar.baz");
    }
}
