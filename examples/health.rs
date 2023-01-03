use libvault_rs::{
    client::vault::VaultHttpClient,
    sys::requests::health_request,
};

#[tokio::main]
async fn main() {
    let vault_client: VaultHttpClient = VaultHttpClient::new(
        "localhost",
        8200,
        false,
        None as Option<String>
    );

    println!("{:#?}", health_request(&vault_client).await);
}
