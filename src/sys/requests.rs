use crate::client::vault::{VaultClient, VaultClientError};

use super::responses::HealthResponse;

pub async fn health_request(vault_client: &dyn VaultClient) -> Result<HealthResponse, VaultClientError> {
    let url = format!("{}/{}", vault_client.base_url(), "sys/health");
    match vault_client.read(url, None).await {
        Err(e) => Err(e),
        Ok(value) => serde_json::from_value(value).map_err(|e| VaultClientError::new(e.to_string())),
    }
}