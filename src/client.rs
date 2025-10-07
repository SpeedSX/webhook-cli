use anyhow::{Context, Result};
use reqwest::{Client, StatusCode};
use std::time::Duration;

use crate::config::Config;
use crate::models::WebhookRequest;

pub struct WebhookClient {
    client: Client,
    base_url: String,
}

impl WebhookClient {
    pub fn new(config: &Config) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: config.get_base_url().to_string(),
        }
    }

    pub async fn get_requests(&self, token: &str, count: u32) -> Result<Vec<WebhookRequest>> {
        let url = Config::join_url_segments(&self.base_url, &[token, "log", &count.to_string()]);

        let response = self
            .client
            .get(&url)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await
            .with_context(|| format!("Failed to fetch requests from {}", url))?;

        let status = response.status();

        if status.is_success() {
            let response_text = response
                .text()
                .await
                .with_context(|| "Failed to read response body")?;

            let requests: Vec<WebhookRequest> =
                serde_json::from_str(&response_text).with_context(|| {
                    format!(
                        "Failed to parse response as JSON. Response body: {}",
                        response_text
                    )
                })?;
            Ok(requests)
        } else if status == StatusCode::NOT_FOUND {
            Ok(vec![]) // No requests yet
        } else {
            let response_body = response
                .text()
                .await
                .unwrap_or_else(|_| "(failed to read response body)".to_string());

            anyhow::bail!(
                "HTTP {} {}: {}",
                status.as_u16(),
                status.canonical_reason().unwrap_or("Unknown"),
                response_body
            );
        }
    }
}
