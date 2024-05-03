# Azure App Config

## Purpose

This library is a quick wrapper around the azure keyvault REST API to support managed identity configuration setting grabbing.

## Usage


### Get All Key Values
```rust

    let client = Client::new(azure_core::Url::parse(env::var("APPCONFIG_URI").expect("Missing APPCONFIG_URI Env Var").as_str()).unwrap(), azure_identity::create_credential().unwrap());
    let resp = client.get_key_values(None, None, None).await;
```

### Get a Specific Config Value

```rust

    let client = Client::new(azure_core::Url::parse(env::var("APPCONFIG_URI").expect("Missing APPCONFIG_URI Env Var").as_str()).unwrap(), azure_identity::create_credential().unwrap());
    let config_setting = client.get_key_value("ExampleConfigSetting", None, Some("Example:"), None).await.unwrap();
    
```

### Get Feature Flags

```rust
    let client = Client::new(azure_core::Url::parse(env::var("APPCONFIG_URI").expect("Missing APPCONFIG_URI Env Var").as_str()).unwrap(), azure_identity::create_credential().unwrap());
    let feature_flags = client.get_feature_flags(None).await;
```

### Get Feature Flag Is Enabled or Disabled

```rust
    let client = Client::new(azure_core::Url::parse(env::var("APPCONFIG_URI").expect("Missing APPCONFIG_URI Env Var").as_str()).unwrap(), azure_identity::create_credential().unwrap());
    let is_enabled = client.is_feature_enabled("ExampleFeatureFlag", None).await.unwrap();
```

Also see the Example: [Basic Usage](examples/basic-usage.rs)

## Install

`cargo add azure_app_config`

### Crates.IO

https://crates.io/crates/azure_app_config