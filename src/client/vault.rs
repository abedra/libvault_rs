use std::collections::HashMap;

use futures::{future::BoxFuture, FutureExt};
use reqwest::{Client, Body, header::HeaderMap};
use serde::Serialize;

#[derive(Debug)]
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

#[derive(Serialize, Default, Debug)]
pub struct Parameters {
    pub value: HashMap<String, ParameterValue>
}

impl Parameters {
    pub fn new(value: HashMap<String, ParameterValue>) -> Self {
        Self { value }
    }
}

impl Into<Body> for Parameters {
    fn into(self) -> Body {
        Body::from(serde_json::to_string(&self.value).unwrap())
    }
}

pub trait VaultClient {
    fn read(&self, url: String, token: Option<String>) -> BoxFuture<serde_json::Value>;
    fn create(&self, url: String, token: Option<String>, parameters: Parameters) -> BoxFuture<serde_json::Value>;
    fn base_url(&self) -> String;
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
        client: Client,
        host: impl Into<String>, 
        port: u16, 
        tls: bool, 
        namespace: Option<impl Into<String>>,
    ) -> Self {
        Self {
            client,
            host: host.into(), 
            port,
            tls,
            namespace: match namespace {
                Some(value) => Some(value.into()),
                None => None,
            }
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
    fn read(&self, url: String, token: Option<String>) -> BoxFuture<serde_json::Value> {
        async {
            self.client.get(url)
                .headers(self.build_headers(token))
                .send().await.unwrap()
                .json().await.unwrap()
        }.boxed()
    }

    fn create(&self, url: String, token: Option<String>, parameters: Parameters) -> BoxFuture<serde_json::Value> {
        async {
            self.client.post(url)
                .headers(self.build_headers(token))
                .body(parameters)
                .send().await.unwrap()
                .json().await.unwrap()
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