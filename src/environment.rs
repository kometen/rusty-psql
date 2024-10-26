//! Environment Module
//!
//! This module gets environment variables from a .env-file
//!

mod tests;

use anyhow::{Context, Result};
use dotenv::dotenv;
use std::env;

pub struct Environment {
    pub domain: String,
}

impl Environment {
    /// Creates an Environment instance with the domain-name of the server.
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// A Result containing the domain name if successful, or an error
    /// if the domain is not defined.
    ///
    /// # Example
    ///
    /// ```
    /// use rusty_psql::Environment;
    /// use anyhow::Result;
    ///
    /// fn example() -> Result<()> {
    ///     let environment = Environment::new()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new() -> Result<Self> {
        dotenv().ok();

        let domain = env::var("DOMAIN").context("Failed to get domain")?;

        Ok(Self { domain })
    }

    #[cfg(test)]
    fn with_domain(domain: Option<String>) -> Result<Self> {
        match domain {
            Some(d) => Ok(Self { domain: d }),
            None => Err(anyhow::anyhow!("Failed to get domain")),
        }
    }
}
