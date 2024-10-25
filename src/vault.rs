use azure_security_keyvault::SecretClient;

pub struct Vault {
    pub host: String,
    pub user: String,
    pub name: String,
    pub pwd: String,
}

impl Vault {
    pub async fn new(url: String) -> Result<Self, std::io::Error> {
        let credential = azure_identity::create_credential().unwrap();
        let client = SecretClient::new(url.as_str(), credential).unwrap();

        let host = get_secret(&client, String::from("db-host")).await;
        let user = get_secret(&client, String::from("db-user")).await;
        let name = get_secret(&client, String::from("db-name")).await;
        let pwd = get_secret(&client, String::from("db-pwd")).await;

        Ok(Self {
            host,
            user,
            name,
            pwd,
        })
    }
}

async fn get_secret(client: &SecretClient, key: String) -> String {
    client
        .get(key.clone())
        .await
        .map_err(|e| format!("Error fetching secret using key {}: {}", key, e))
        .unwrap()
        .value
}
