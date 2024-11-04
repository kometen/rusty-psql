pub mod connect;

pub use connect::connect::run_psql;

use azure_vault_secrets::VaultStorage;
use db_config::db_config_from_vault;

db_config_from_vault!([host, user, name, pwd, domain]);
