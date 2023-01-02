use super::responses::{KeyValueResponse, KeyValueV1Response, KeyValueV2Response};
use crate::client::vault::{AuthenticatedVaultClient, VaultClientError};

pub enum KeyValueVersion {
    One,
    Two,
}

pub struct KeyValue {
    version: KeyValueVersion,
    mount: String,
}

impl KeyValue {
    pub fn new(version: KeyValueVersion, mount: impl Into<String>) -> Self {
        Self {
            version,
            mount: mount.into(),
        }
    }

    pub async fn read(
        &self,
        vault_client: &AuthenticatedVaultClient,
        path: &str,
    ) -> Result<KeyValueResponse, VaultClientError> {
        let response = vault_client
            .client
            .read(
                self.url(vault_client.client.base_url(), path),
                Some(vault_client.token.clone()),
            )
            .await;

        match response {
            Err(e) => Err(e),
            Ok(value) => {
                match self.version {
                    KeyValueVersion::One => {
                        let convert: Result<KeyValueV1Response, serde_json::Error> =
                            serde_json::from_value(value);
                        match convert {
                            Ok(value) => Ok(KeyValueResponse::new(value.data)),
                            Err(e) => Err(VaultClientError::new(e.to_string())),
                        }
                    }
                    KeyValueVersion::Two => {
                        let convert: Result<KeyValueV2Response, serde_json::Error> =
                            serde_json::from_value(value);
                        match convert {
                            Ok(value) => Ok(KeyValueResponse::new(value.data.data)),
                            Err(e) => Err(VaultClientError::new(e.to_string())),
                        }
                    }
                }                        
            },
        }
    }

    fn url(&self, base_url: String, path: &str) -> String {
        match self.version {
            KeyValueVersion::One => format!("{}/{}/{}", base_url, self.mount, path),
            KeyValueVersion::Two => format!("{}/{}/data/{}", base_url, self.mount, path),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{KeyValue, KeyValueVersion};
    use crate::{
        client::vault::{AuthenticatedVaultClient, Parameters, VaultClient, VaultClientError},
        secrets::key_value::{
            test_helpers::{
                successful_key_value_response, successful_v1_json_response,
                successful_v2_json_response,
            },
        },
    };
    use futures::{future::BoxFuture, FutureExt};
    use serde_json::json;
    

    struct StubClient {
        response: serde_json::Value,
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
    async fn v1_success() {
        let vault_client: StubClient = StubClient {
            response: successful_v1_json_response(),
        };
        let key_value: KeyValue = KeyValue::new(KeyValueVersion::One, "secrets");
        let authenticated_client: AuthenticatedVaultClient =
            AuthenticatedVaultClient::new(Box::new(vault_client), "token".into());

        assert_eq!(
            Ok(successful_key_value_response()),
            key_value.read(&authenticated_client, "test").await
        );
    }

    #[tokio::test]
    async fn v2_success() {
        let vault_client: StubClient = StubClient {
            response: successful_v2_json_response(),
        };
        let key_value: KeyValue = KeyValue::new(KeyValueVersion::Two, "secrets");
        let authenticated_client: AuthenticatedVaultClient =
            AuthenticatedVaultClient::new(Box::new(vault_client), "token".into());

        assert_eq!(
            Ok(successful_key_value_response()),
            key_value.read(&authenticated_client, "test").await
        );
    }

    #[tokio::test]
    async fn v1_deserialization_failure() {
        let vault_client: StubClient = StubClient {
            response: json!({}),
        };
        let key_value: KeyValue = KeyValue::new(KeyValueVersion::One, "secrets");
        let authenticated_client: AuthenticatedVaultClient =
            AuthenticatedVaultClient::new(Box::new(vault_client), "token".into());

        assert_eq!(
            Err(VaultClientError::new("missing field `data`".into())),
            key_value.read(&authenticated_client, "test").await
        );
    }

    #[tokio::test]
    async fn v2_deserialization_failure() {
        let vault_client: StubClient = StubClient {
            response: json!({}),
        };
        let key_value: KeyValue = KeyValue::new(KeyValueVersion::Two, "secrets");
        let authenticated_client: AuthenticatedVaultClient =
            AuthenticatedVaultClient::new(Box::new(vault_client), "token".into());

        assert_eq!(
            Err(VaultClientError::new("missing field `data`".into())),
            key_value.read(&authenticated_client, "test").await
        );
    }
}
