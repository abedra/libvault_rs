use libvault_rs::{client::vault::{VaultHttpClient, VaultClient}, sys::{requests::health_request, responses::HealthResponse}};
use reqwest::Client;

#[tokio::main]
async fn main() {
    let client: Client = Client::new();
    let vault_client: VaultHttpClient = VaultHttpClient::new(
        client,
        "localhost",
        8200,
        false,
        None as Option<String>
    );
    let health_response = health_request(&vault_client).await;
    println!("{:#?}", health_response);
}