use std::collections::HashMap;
use serde::Serialize;
use crate::client::vault::{VaultClient, Parameters, ParameterValue, VaultClientError};
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

impl From<ApproleCredentials> for Parameters {
    fn from(approle_credentials: ApproleCredentials) -> Self {
        Parameters::new(HashMap::from([
            ("role_id", ParameterValue::String(approle_credentials.role_id)),
            ("secret_id", ParameterValue::String(approle_credentials.secret_id))
        ]))
    }
}

pub async fn login(
    vault_client: &impl VaultClient,
    approle_credentials: ApproleCredentials
) -> Result<ApproleLoginResponse, VaultClientError> {
    let url = format!("{}/{}", vault_client.base_url(), "auth/approle/login");
    let response = vault_client.create(url, None, approle_credentials.into()).await;
    match response {
        Err(e) => Err(e),
        Ok(value) => serde_json::from_value(value).map_err(|e| VaultClientError::new(e.to_string())),
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use futures::{future::BoxFuture, FutureExt};
    use serde_json::json;
    use crate::{client::vault::{VaultClient, Parameters, ParameterValue, VaultClientError}, auth::approle::{test_helpers::{successful_login_response, successful_login_json_response}}};
    use super::{ApproleCredentials, login};

    #[derive(Default)]
    struct ApproleClient {
        response: serde_json::Value
    }
    
    impl VaultClient for ApproleClient {
        fn read(&self, _: String, _: Option<String>) -> BoxFuture<Result<serde_json::Value, VaultClientError>> {
            unimplemented!()
        }

        fn create(&self, _: String, _: Option<String>, _: crate::client::vault::Parameters) -> BoxFuture<Result<serde_json::Value, VaultClientError>> {
            async { Ok(self.response.to_owned()) }.boxed()
        }

        fn base_url(&self) -> String {
            "test".to_string()
        }
    }

    #[test]
    fn parameterization() {
        let approle_credentials: ApproleCredentials = ApproleCredentials::new("role_id", "secret_id");
        let expected: Parameters = Parameters::new(HashMap::from([
            ("role_id", ParameterValue::String("role_id".into())),
            ("secret_id", ParameterValue::String("secret_id".into()))
        ]));

        assert_eq!(expected, approle_credentials.into());
    }

    #[tokio::test]
    async fn deserialization_failure() {
        let vault_client: ApproleClient = ApproleClient { response: json!({}) };
        let approle_credentials: ApproleCredentials = ApproleCredentials::new("role", "secret");

        assert_eq!(
            Err(VaultClientError::new("missing field `auth`".into())), 
            login(&vault_client, approle_credentials).await
        );
    }

    #[tokio::test]
    async fn login_successful() {
        let vault_client: ApproleClient = ApproleClient { response: successful_login_json_response() };
        let approle_credentials: ApproleCredentials = ApproleCredentials::new("role", "secret");
        
        assert_eq!(
            Ok(successful_login_response()), 
            login(&vault_client, approle_credentials).await
        );
    }
}