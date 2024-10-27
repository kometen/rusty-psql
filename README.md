# rusty_psql

A wrapper around psql so information about host to connect to, database, password is retrieved from Azure Key Vault
and URL to Azure Key Vault from 1password thus avoiding storing passwords in cleartext locally.

The command line utilities from 1password and Azure are required, as well as Azure Key Vault. Log in with `az login`.

Clone the repository, run `cargo build [--release]`, add the `.env`file with the domain where the PostgreSQL database is.

Add the following secrets to Azure Key Vault:
```
db-host
db-name
db-pwd
db-user
db-domain
```

Add an entry in 1password that can be accessed by the program. The path is formatted in Rust.

```
let op_path = format!("op://Production/AzureKeyVault{}/credentials/url", key);
```

An example of a path can be `op://Production/AzureKeyVaultInvoice/credentials/url`.

Build and run the program

```
./target/release/rusty_psql -n invoice
```

Build and run the container-image.

```
docker build -t rusty_psql:dev .
docker run --user rusty_psql_user backup_db:dev
```

The container will not inherit the environment from the shell so running from the container will not work.

I had much help from Claude. Used the editor zed at https://github.com/zed-industries/zed.
