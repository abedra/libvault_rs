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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use futures::{future::BoxFuture, FutureExt};
    use serde_json::json;
    use crate::{client::vault::{VaultClient, Parameters, ParameterValue}, auth::approle::responses::{ApproleLoginResponse, Auth}};
    use super::{ApproleCredentials, login};

    #[derive(Default)]
    struct ApproleClient {}
    impl VaultClient for ApproleClient {
        fn read(&self, _: String, _: Option<String>) -> BoxFuture<serde_json::Value> {
            unimplemented!()
        }

        fn create(&self, _: String, _: Option<String>, _: crate::client::vault::Parameters) -> BoxFuture<serde_json::Value> {
            let response = json!(
                {
                    "auth": {
                      "accessor": "LsKyvlMAvGcQhktNAjZ9it8q",
                      "client_token": "hvs.CAESINqsFsEB_CyrfrTzkOLf_eZbOpcvAlAr5Kh8ulWb6_HmGh4KHGh2cy5DclY5bm9aakJrMDBheTExYU1TSVQxanQ",
                      "entity_id": "4dc20fd9-71e4-b61e-5907-f5926ccbf964",
                      "lease_duration": 2764800,
                      "metadata": {
                        "role_name": "client"
                      },
                      "mfa_requirement": null,
                      "num_uses": 0,
                      "orphan": true,
                      "policies": [
                        "default",
                        "example"
                      ],
                      "renewable": true,
                      "token_policies": [
                        "default",
                        "example"
                      ],
                      "token_type": "service"
                    },
                    "data": null,
                    "lease_duration": 0,
                    "lease_id": "",
                    "renewable": false,
                    "request_id": "8ff6e17e-61b0-7a65-dcbc-6870b1fd8d1e",
                    "warnings": null,
                    "wrap_info": null
                  }                  
            );

            async { response }.boxed()
        }

        fn base_url(&self) -> String {
            "test".to_string()
        }
    }

    #[test]
    fn parameterization() {
        let approle_credentials: ApproleCredentials = ApproleCredentials::new("role_id", "secret_id");
        let expected: Parameters = Parameters::new(HashMap::from([("role_id", ParameterValue::String("role_id".into())), ("secret_id", ParameterValue::String("secret_id".into()))]));
        let actual: Parameters = approle_credentials.into();

        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn login_successful() {
        let vault_client: ApproleClient = ApproleClient::default();
        let approle_credentials: ApproleCredentials = ApproleCredentials::new("role", "secret");
        let expected: ApproleLoginResponse = ApproleLoginResponse { 
            auth: Auth { 
                accessor: "LsKyvlMAvGcQhktNAjZ9it8q".into(),
                client_token: "hvs.CAESINqsFsEB_CyrfrTzkOLf_eZbOpcvAlAr5Kh8ulWb6_HmGh4KHGh2cy5DclY5bm9aakJrMDBheTExYU1TSVQxanQ".into(),
                entity_id: "4dc20fd9-71e4-b61e-5907-f5926ccbf964".into(),
                lease_duration: 2764800,
                metadata: HashMap::from([("role_name".into(), "client".into())]),
                mfa_requirement: None,
                num_uses: 0,
                orphan: true,
                policies: vec!["default".into(), "example".into()],
                renewable: true,
                token_policies: vec!["default".into(), "example".into()],
                token_type: "service".into() 
            },
            data: None,
            lease_duration: 0,
            lease_id: "".into(),
            renewable: false,
            request_id: "8ff6e17e-61b0-7a65-dcbc-6870b1fd8d1e".into(),
            warnings: None,
            wrap_info: None
        };
        let actual = login(&vault_client, approle_credentials).await;
        
        assert_eq!(expected, actual);
    }
}