# Azure App Config

## Purpose

This library is a quick wrapper around the azure keyvault REST API to support managed identity configuration setting grabbing.

## Usage

```rust

    let client = Client::new(azure_core::Url::parse(env::var("APPCONFIG_URI").expect("Missing APPCONFIG_URI Env Var").as_str()).unwrap(), azure_identity::create_credential().unwrap());
    let resp = client.get_key_values(None, None, None).await;

```

Also see the Example: [Basic Usage](examples/basic-usage.rs)

## Install

`cargo add azure-app-config`

### Crates.IO

https://crates.io/crates/azure-app-config