use crate::client::vault::AuthenticatedVaultClient;
use super::responses::{KeyValueV1Response, KeyValueResponse, KeyValueV2Response};

pub enum KeyValueVersion {
    One,
    Two
}

pub struct KeyValue {
    version: KeyValueVersion,
    mount: String
}

impl KeyValue {
    pub fn new(version: KeyValueVersion, mount: impl Into<String>) -> Self {
        Self { version, mount: mount.into() }
    }

    pub async fn read(&self, vault_client: &AuthenticatedVaultClient, path: &str) -> KeyValueResponse {
        let response = vault_client.client.read(self.url(vault_client.client.base_url(), path), Some(vault_client.token.clone())).await;
        match self.version {
            KeyValueVersion::One => {
                let derived: KeyValueV1Response = serde_json::from_value(response).unwrap();
                KeyValueResponse::new(derived.data)
            },
            KeyValueVersion::Two => {
                let derived: KeyValueV2Response = serde_json::from_value(response).unwrap();
                KeyValueResponse::new(derived.data.data)
            }
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
    use std::collections::HashMap;
    use futures::{future::BoxFuture, FutureExt};
    use serde_json::json;
    use crate::{client::vault::{VaultClient, Parameters, AuthenticatedVaultClient}, secrets::key_value::responses::KeyValueResponse};
    use super::{KeyValue, KeyValueVersion};

    #[derive(Default)]
    struct V1Client {}
    #[derive(Default)]
    struct V2Client {}

    impl VaultClient for V1Client {
        fn read(&self, _: String, _: Option<String>) -> BoxFuture<serde_json::Value> {
            async { 
                json!({
                    "auth": null,
                    "data": {
                    "foo": "bar"
                    },
                    "lease_duration": 2764800,
                    "lease_id": "",
                    "renewable": false,
                    "request_id": "782a5b1b-8ec3-3e47-b2e5-e0a43fb6ad5f",
                    "warnings": null,
                    "wrap_info": null
                }) 
            }.boxed()
        }

        fn create(&self, _: String, _: Option<String>, _: Parameters) -> BoxFuture<serde_json::Value> {
            unimplemented!()
        }

        fn base_url(&self) -> String {
            "test".into()
        }
    }

    impl VaultClient for V2Client {
        fn read(&self, _: String, _: Option<String>) -> BoxFuture<serde_json::Value> {
            async { 
                json!({
                    "auth": null,
                    "data": {
                      "data": {
                        "foo": "bar"
                      },
                      "metadata": {
                        "created_time": "2023-01-01T20:36:17.398127031Z",
                        "custom_metadata": null,
                        "deletion_time": "",
                        "destroyed": false,
                        "version": 1
                      }
                    },
                    "lease_duration": 0,
                    "lease_id": "",
                    "renewable": false,
                    "request_id": "630bd562-0379-18c5-d3a1-dddccf8bf3ec",
                    "warnings": null,
                    "wrap_info": null
                }) 
            }.boxed()
        }

        fn create(&self, _: String, _: Option<String>, _: Parameters) -> BoxFuture<serde_json::Value> {
            unimplemented!()
        }

        fn base_url(&self) -> String {
            "test".into()
        }
    }

    #[tokio::test]
    async fn v1_success() {
        let vault_client: V1Client = V1Client::default();
        let authenticated_client: AuthenticatedVaultClient = AuthenticatedVaultClient::new(Box::new(vault_client), "token".into());
        let key_value: KeyValue = KeyValue::new(KeyValueVersion::One, "secrets");
        let expected: KeyValueResponse = KeyValueResponse { 
            data: HashMap::from([("foo".into(), "bar".into())])
        };
        let actual: KeyValueResponse = key_value.read(&authenticated_client, "test").await;

        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn v2_success() {
        let vault_client: V2Client = V2Client::default();
        let authenticated_client: AuthenticatedVaultClient = AuthenticatedVaultClient::new(Box::new(vault_client), "token".into());
        let key_value: KeyValue = KeyValue::new(KeyValueVersion::Two, "secrets");
        let expected: KeyValueResponse = KeyValueResponse { 
            data: HashMap::from([("foo".into(), "bar".into())])
        };
        let actual: KeyValueResponse = key_value.read(&authenticated_client, "test").await;

        assert_eq!(expected, actual);
    }
}