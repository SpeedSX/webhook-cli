use anyhow::{Context, Result};
use reqwest::Client;

use crate::config::Config;
use crate::models::WebhookRequest;

pub struct WebhookClient {
    client: Client,
    base_url: String,
}

impl WebhookClient {
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            base_url: config.get_base_url().to_string(),
        }
    }

    pub async fn get_requests(&self, token: &str, count: u32) -> Result<Vec<WebhookRequest>> {
        let url = format!("{}/{}/log/{}", self.base_url, token, count);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("Failed to fetch requests from {}", url))?;

        if response.status().is_success() {
            let requests: Vec<WebhookRequest> = response
                .json()
                .await
                .with_context(|| "Failed to parse response as JSON")?;
            Ok(requests)
        } else if response.status() == 404 {
            Ok(vec![]) // No requests yet
        } else {
            anyhow::bail!("HTTP {}: {}", response.status(), response.status());
        }
    }
}