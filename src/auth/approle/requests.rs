use std::collections::HashMap;
use serde::Serialize;
use crate::client::vault::{VaultClient, Parameters, ParameterValue};
use super::responses::ApproleLoginResponse;

#[derive(Debug, Serialize)]
pub struct ApproleCredentials {
    role_id: String,
    secret_id: String
}

impl ApproleCredentials {
    pub fn new(role_id: impl Into<String>, secret_id: impl Into<String>) -> Self {
        Self { role_id: role_id.into(), secret_id: secret_id.into() }
    }
}

impl Into<Parameters> for ApproleCredentials {
    fn into(self) -> Parameters {
        Parameters::new(HashMap::from([
            ("role_id", ParameterValue::String(self.role_id)),
            ("secret_id", ParameterValue::String(self.secret_id))
        ]))
    }
}

pub async fn login(vault_client: &impl VaultClient, approle_credentials: ApproleCredentials) -> ApproleLoginResponse {
    let url = format!("{}/{}", vault_client.base_url(), "auth/approle/login");
    let response = vault_client.create(url, None, approle_credentials.into()).await;
    serde_json::from_value(response).unwrap()
}