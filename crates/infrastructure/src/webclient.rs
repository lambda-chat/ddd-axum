use domain::error::DomainError;
use reqwest::Client;

#[derive(Clone)]
pub struct ExternalApiClient {
    client: Client,
    base_url: String,
}

impl ExternalApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
        }
    }

    pub async fn fetch_data(&self, path: &str) -> Result<String, DomainError> {
        let url = format!("{}/{}", self.base_url, path);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| DomainError::ExternalApiError(e.to_string()))?;
        let body = resp
            .text()
            .await
            .map_err(|e| DomainError::ExternalApiError(e.to_string()))?;
        Ok(body)
    }
}
