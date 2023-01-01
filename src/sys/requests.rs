use crate::client::vault::VaultClient;

use super::responses::HealthResponse;

pub async fn health_request(vault_client: &dyn VaultClient) -> HealthResponse {
    let url = format!("{}/{}", vault_client.base_url(), "sys/health");
    serde_json::from_value(vault_client.read(url).await).unwrap()
}