use libvault_rs::{
    client::vault::VaultHttpClient,
    sys::requests::{health, leader},
};

#[tokio::main]
async fn main() {
    let vault_client: VaultHttpClient = VaultHttpClient::new(
        "localhost",
        8200,
        false,
        None as Option<String>
    );

    println!("{:#?}", health(&vault_client).await);
    println!("{:#?}", leader(&vault_client).await);
}
