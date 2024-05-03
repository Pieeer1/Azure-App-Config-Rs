use std::{env, error::Error};

use azure_app_config::azure_app_config::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let client = Client::new(azure_core::Url::parse(env::var("APPCONFIG_URI").expect("Missing APPCONFIG_URI Env Var").as_str()).unwrap(), azure_identity::create_credential().unwrap());

    println!("keys: ");

    let keys = client.get_keys(None).await;

    keys.unwrap().as_vec().iter().for_each(|k| {
        println!("{}", k);
    });

    println!("key values: ");

    let key_values = client.get_key_values(None, None, None).await;

    key_values.unwrap().as_hash_map().iter().for_each(|(k, v)| {
        println!("{}: {}", k, v);
    });

    println!("example config setting");

    let config_setting = client.get_key_value("ExampleConfigSetting", None, Some("Example:"), None).await.unwrap();

    println!("{}: {}", config_setting.key, config_setting.value);

    println!("feature flags: ");

    let feature_flags = client.get_feature_flags(None).await;

    feature_flags.unwrap().items.iter().for_each(|f| {
        println!("{}: {}", f.id, f.enabled);
    });

    println!("example feature flag:");

    let example_feature_flag = client.is_feature_enabled("ExampleFeatureFlag", None).await.unwrap();

    println!("{}", example_feature_flag);

    return Ok(());
}