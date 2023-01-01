use futures::{future::BoxFuture, FutureExt};

pub trait VaultClient {
    fn request(&self, url: String) -> BoxFuture<serde_json::Value>;
}

pub struct VaultHttpClient {
    host: String,
    port: u16,
    tls: bool,
    verify_tls: bool,
    namespace: String
}

impl VaultHttpClient {
    pub fn new(
        host: impl Into<String>, 
        port: u16, 
        tls: bool, 
        verify_tls: bool,
        namespace: impl Into<String>,
    ) -> Self {
        Self { 
            host: host.into(), 
            port,
            tls,
            verify_tls,
            namespace: namespace.into()
        }
    }

    pub fn base_url(&self) -> String {
        let protocol = match self.tls {
            true => "https",
            false => "http",
        };
        format!("{}://{}:{}/v1", protocol, self.host, self.port)
    }
}

impl VaultClient for VaultHttpClient {
    fn request(&self, url: String) -> BoxFuture<serde_json::Value> {
        async {
            let response: serde_json::Value = reqwest::get(url).await.unwrap().json().await.unwrap();
            response
        }.boxed()
    }
}
