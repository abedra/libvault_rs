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
        let mut parameter_map: HashMap<&str, ParameterValue> = HashMap::new();
        parameter_map.insert("role_id", ParameterValue::String(self.role_id));
        parameter_map.insert("secret_id", ParameterValue::String(self.secret_id));
        Parameters::new(parameter_map)
    }
}

pub async fn login(vault_client: &dyn VaultClient, approle_credentials: ApproleCredentials) -> ApproleLoginResponse {
    let url = format!("{}/{}", vault_client.base_url(), "auth/approle/login");
    let response = vault_client.create(url, None, approle_credentials.into()).await;
    serde_json::from_value(response).unwrap()
}