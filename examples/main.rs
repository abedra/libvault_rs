use libvault_rs::{client::vault::{VaultHttpClient, VaultClient}, sys::{requests::health_request, responses::HealthResponse}};

#[tokio::main]
async fn main() {
    let client: VaultHttpClient = VaultHttpClient::new(
        "localhost",
        8200,
        false,
        false
    );
    let url = health_request(client.base_url());
    let response = client.request(url);
    let health_response: HealthResponse = serde_json::from_value(response.await).unwrap();
    println!("{:#?}", health_response);
}