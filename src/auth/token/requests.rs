use crate::client::vault::VaultClient;

pub async fn list(vault_client: &dyn VaultClient, token: String) -> serde_json::Value {
    let url = format!("{}/{}", vault_client.base_url(), "auth/token/accessors");
    println!("{}", url);
    vault_client.read(url, Some(token)).await
}