use std::collections::HashMap;
use futures::{future::BoxFuture, FutureExt};
use reqwest::{Client, Body, header::HeaderMap};
use serde::Serialize;

#[derive(Debug, PartialEq)]
pub struct VaultClientError {
    pub error: String
}

impl VaultClientError {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParameterValue {
    Int(i32),
    Str(&'static str),
    String(String)
}

impl Serialize for ParameterValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        match self {
            ParameterValue::Int(value) => serializer.serialize_i32(*value),
            ParameterValue::Str(value) => serializer.serialize_str(value),
            ParameterValue::String(value) => serializer.serialize_str(value),
        }
    }
}

#[derive(Serialize, Default, Debug, PartialEq)]
pub struct Parameters {
    pub value: HashMap<&'static str, ParameterValue>
}

impl Parameters {
    pub fn new(value: HashMap<&'static str, ParameterValue>) -> Self {
        Self { value }
    }
}

impl From<Parameters> for Body {
    fn from(parameters: Parameters) -> Self {
        Body::from(serde_json::to_string(&parameters.value).unwrap())
    }
}

pub trait VaultClient {
    fn read(&self, url: String, token: Option<String>) -> BoxFuture<Result<serde_json::Value, VaultClientError>>;
    fn create(&self, url: String, token: Option<String>, parameters: Parameters) -> BoxFuture<Result<serde_json::Value, VaultClientError>>;
    fn base_url(&self) -> String;
}

pub struct AuthenticatedVaultClient {
    pub client: Box<dyn VaultClient>,
    pub token: String
}

impl AuthenticatedVaultClient {
    pub fn new(client: Box<dyn VaultClient>, token: String) -> Self {
        Self { client, token }
    }
}

pub struct VaultHttpClient {
    client: Client,
    host: String,
    port: u16,
    tls: bool,
    namespace: Option<String>
}

impl VaultHttpClient {
    pub fn new(
        host: impl Into<String>, 
        port: u16, 
        tls: bool, 
        namespace: Option<impl Into<String>>,
    ) -> Self {
        Self {
            client: Client::new(),
            host: host.into(), 
            port,
            tls,
            namespace: namespace.map(|value| value.into())
        }
    }

    fn build_headers(&self, token: Option<String>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Some(token) = token {
            headers.insert("X-Vault-Token", token.parse().unwrap());
        }
        if let Some(namespace) = &self.namespace {
            headers.insert("X-Vault-Namespace", namespace.parse().unwrap());
        }
        headers        
    }
}

impl VaultClient for VaultHttpClient {
    fn read(&self, url: String, token: Option<String>) -> BoxFuture<Result<serde_json::Value, VaultClientError>> {
        async {
            match self.client.get(url).headers(self.build_headers(token)).send().await {
                Ok(v) => {
                    match v.json::<serde_json::Value>().await {
                        Ok(v) => Ok(v),
                        Err(e) => Err(VaultClientError::new(e.to_string())),
                    }
                },
                Err(e) => Err(VaultClientError::new(e.to_string())),
            }
        }.boxed()
    }

    fn create(&self, url: String, token: Option<String>, parameters: Parameters) -> BoxFuture<Result<serde_json::Value, VaultClientError>> {
        async {
            match self.client.post(url).headers(self.build_headers(token)).body(parameters).send().await {
                Ok(v) => {
                    match v.json::<serde_json::Value>().await {
                        Ok(v) => Ok(v),
                        Err(e) => Err(VaultClientError::new(e.to_string())),
                    }
                },
                Err(e) => Err(VaultClientError::new(e.to_string())),
            }            
        }.boxed()
    }

    fn base_url(&self) -> String {
        let protocol = match self.tls {
            true => "https",
            false => "http",
        };
        format!("{}://{}:{}/v1", protocol, self.host, self.port)
    }
}
