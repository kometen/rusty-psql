#[cfg(test)]
mod tests {
    use crate::Environment;

    #[test]
    fn test_error_is_returned_when_domain_is_not_set() {
        let env_1 = Environment::with_domain(None);
        assert!(env_1.is_err());
    }

    #[test]
    fn test_ok_is_returned_when_domain_is_set() {
        let env_2 = Environment::with_domain(Some("foo.bar.baz".to_string()));
        assert_eq!(env_2.unwrap().domain, "foo.bar.baz");
    }
}
