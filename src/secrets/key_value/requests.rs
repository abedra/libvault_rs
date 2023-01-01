use crate::client::vault::VaultClient;

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

    pub async fn read(&self, vault_client: &dyn VaultClient, token: String, path: &str) -> KeyValueResponse {
        let response = vault_client.read(self.url(vault_client, path), Some(token)).await;
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

    fn url(&self, client: &dyn VaultClient, path: &str) -> String {
        match self.version {
            KeyValueVersion::One => format!("{}/{}/{}", client.base_url(), self.mount, path),
            KeyValueVersion::Two => format!("{}/{}/data/{}", client.base_url(), self.mount, path),
        }
    }
}
