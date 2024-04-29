use std::{env, error::Error};

use azure_app_config::azure_app_config::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let client = Client::new(azure_core::Url::parse(env::var("APPCONFIG_URI").expect("Missing APPCONFIG_URI Env Var").as_str()).unwrap(), azure_identity::create_credential().unwrap());

    let keys = client.get_keys(None).await;

    keys.unwrap().as_vec().iter().for_each(|k| {
        println!("{}", k);
    });

    let resp = client.get_key_values(None, None, None).await;

    resp.unwrap().as_hash_map().iter().for_each(|(k, v)| {
        println!("{}: {}", k, v);
    });

    return Ok(());
}