use crate::client::vault::{VaultClient, VaultClientError};
use super::responses::{HealthResponse, LeaderResponse};

pub async fn health(vault_client: &dyn VaultClient) -> Result<HealthResponse, VaultClientError> {
    let url: String = format!("{}/{}", vault_client.base_url(), "sys/health");
    match vault_client.read(url, None).await {
        Err(e) => Err(e),
        Ok(value) => serde_json::from_value(value).map_err(|e| VaultClientError::new(e.to_string())),
    }
}

pub async fn leader(vault_client: &dyn VaultClient) -> Result<LeaderResponse, VaultClientError> {
    let url: String = format!("{}/{}", vault_client.base_url(), "sys/leader");
    match vault_client.read(url, None).await {
        Err(e) => Err(e),
        Ok(value) => serde_json::from_value(value).map_err(|e| VaultClientError::new(e.to_string())),
    }
}

#[cfg(test)]
mod test {
    use futures::{FutureExt, future::BoxFuture};
    use crate::{client::vault::{VaultClient, VaultClientError, Parameters}, sys::{test_helpers::{health_success, health_success_json, leader_success_json, leader_success}, requests::{health, leader}}};

    struct StubClient {
        response: serde_json::Value
    }

    impl VaultClient for StubClient {
        fn read(&self, _: String, _: Option<String>) -> BoxFuture<Result<serde_json::Value, VaultClientError>> {
            async { Ok(self.response.to_owned()) }.boxed()
        }

        fn create(&self, _: String, _: Option<String>, _: Parameters) -> BoxFuture<Result<serde_json::Value, VaultClientError>> {
            unimplemented!()
        }

        fn base_url(&self) -> String {
            "test".into()
        }
    }

    #[tokio::test]
    async fn successful_health_response() {
        let vault_client: StubClient = StubClient { 
            response: health_success_json() 
        };

        assert_eq!(
            Ok(health_success()),
            health(&vault_client).await
        );
    }

    #[tokio::test]
    async fn successful_leader_response() {
        let vault_client: StubClient = StubClient { 
            response: leader_success_json() 
        };

        assert_eq!(
            Ok(leader_success()),
            leader(&vault_client).await
        );
    }
}