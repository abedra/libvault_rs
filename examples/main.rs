use std::env;

use vaultrs::{
    client::vault::VaultHttpClient, 
    auth::approle::requests::{ApproleCredentials, login}, 
    secrets::key_value::requests::{KeyValue, KeyValueVersion}
};
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
    
    let role_id = env::var("ROLE_ID").expect("ROLE_ID not set");
    let secret_id = env::var("SECRET_ID").expect("SECRET_ID not set");
    let approle_credentials = ApproleCredentials::new(role_id, secret_id);
    let response = login(&vault_client, approle_credentials).await;
    let token: String = response.auth.client_token;
    
    let key_value: KeyValue = KeyValue::new(KeyValueVersion::Two, "secret");
    println!("{:#?}", key_value.read(&vault_client, token.clone(), "hello").await);

    let key_value_legacy: KeyValue = KeyValue::new(KeyValueVersion::One, "legacy");
    println!("{:#?}", key_value_legacy.read(&vault_client, token.clone(), "hello").await);
}