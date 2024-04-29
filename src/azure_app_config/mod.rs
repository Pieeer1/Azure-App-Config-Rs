use azure_core::Error;

use self::models::{KeySet, KeyValueSet};

pub mod models;

pub struct Client{
    endpoint: azure_core::Url,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>
}

impl Client {
    pub fn new(endpoint: azure_core::Url, credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            endpoint,
            credential
        }
    }

    pub async fn get_keys(&self, api_version: Option<&str>) -> Result<KeySet, Box<dyn std::error::Error>> {
        let request_endpoint = self.endpoint.join(["/keys", "?api-version=", api_version.unwrap_or("1.0")].concat().as_str())?;

        let request = azure_core::Request::new(request_endpoint, azure_core::Method::Get );

        let response = self.make_request(request).await?;

        let status_code = response.status();
        if !status_code.is_success(){
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other,format!("Failed to get keys from Azure App Configuration service: {}", response.status()))));
        }

        let body = response.json::<KeySet>().await?;

        Ok(body)
    }

    pub async fn get_key_values(&self, label_filter: Option<&str>, key_filter: Option<&str>, api_version: Option<&str>) -> Result<KeyValueSet, Box<dyn std::error::Error>> {
        let request_endpoint = self.endpoint.join(["/kv", "?key=", key_filter.unwrap_or("*"), "&label=", label_filter.unwrap_or("*"), "&api-version=", api_version.unwrap_or("1.0")].concat().as_str())?;

        let request = azure_core::Request::new(request_endpoint, azure_core::Method::Get );

        let response = self.make_request(request).await?;
        
        let status_code = response.status();
        if !status_code.is_success(){
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to get key values from Azure App Configuration service: {}", response.status()))));
        }

        let body = response.json::<KeyValueSet>().await?;

        Ok(body)
    }

    async fn make_request(&self, mut request: azure_core::Request) -> Result<azure_core::Response, Error>{
        let scope_uri = [self.endpoint.as_str(), ".default"].concat();
        let scopes = &[scope_uri.as_str()];

        let access_token = self.credential.get_token(scopes).await.unwrap();

        request.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", access_token.token.secret().to_string()));
        request.insert_header(azure_core::headers::CONTENT_TYPE, "application/json");

        let http_client = azure_core::new_http_client();

        http_client.execute_request(&request).await
    }
}
