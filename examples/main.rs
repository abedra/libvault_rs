use std::env;

use libvault_rs::{
    client::vault::{VaultHttpClient, AuthenticatedVaultClient}, 
    auth::approle::requests::{ApproleCredentials, login}, 
    secrets::key_value::requests::{KeyValue, KeyValueVersion}
};

#[tokio::main]
async fn main() {
    let vault_client: VaultHttpClient = VaultHttpClient::new(
        "localhost",
        8200,
        false,
        None as Option<String>
    );
    
    let role_id = env::var("ROLE_ID").expect("ROLE_ID not set");
    let secret_id = env::var("SECRET_ID").expect("SECRET_ID not set");
    let approle_credentials = ApproleCredentials::new(role_id, secret_id);
    let response = login(&vault_client, approle_credentials).await;
    let token: String = response.auth.client_token;
    let authenticated_client: AuthenticatedVaultClient = AuthenticatedVaultClient::new(Box::new(vault_client), token);
    
    let key_value: KeyValue = KeyValue::new(KeyValueVersion::Two, "secret");
    println!("{:#?}", key_value.read(&authenticated_client, "hello").await);

    let key_value_legacy: KeyValue = KeyValue::new(KeyValueVersion::One, "legacy");
    println!("{:#?}", key_value_legacy.read(&authenticated_client, "hello").await);
}