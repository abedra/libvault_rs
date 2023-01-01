use futures::{future::BoxFuture, FutureExt};
use reqwest::Client;

pub trait VaultClient {
    fn read(&self, url: String) -> BoxFuture<serde_json::Value>;
    
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
}

impl VaultClient for VaultHttpClient {
    fn read(&self, url: String) -> BoxFuture<serde_json::Value> {
        async {
            let mut request = self.client.get(url);
            request = match &self.namespace {
                Some(value) => request.header("X-Vault-Namespace", value),
                None => request,
            };
            request.send().await.unwrap().json().await.unwrap()
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
