use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub webhook: WebhookConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookConfig {
    pub base_url: String,
    pub default_count: u32,
    pub default_interval: u64,
    pub show_headers_by_default: bool,
    pub show_full_body_by_default: bool,
    pub body_preview_length: usize,
}

impl Config {
    pub fn load() -> Result<Self> {
        // Try to load from local config first, then fall back to default config
        let config_paths = ["config.local.toml", "config.toml"];

        for path in config_paths {
            if Path::new(path).exists() {
                let content = fs::read_to_string(path)
                    .with_context(|| format!("Failed to read config file: {}", path))?;

                let config: Config = toml::from_str(&content)
                    .with_context(|| format!("Failed to parse config file: {}", path))?;

                return Ok(config);
            }
        }

        // If no config file exists, create a default one and return default values
        let default_config = Config {
            webhook: WebhookConfig {
                base_url: "https://your-webhook-service.com".to_string(),
                default_count: 10,
                default_interval: 3,
                show_headers_by_default: false,
                show_full_body_by_default: false,
                body_preview_length: 80,
            },
        };

        // Create the default config file
        let default_content = toml::to_string_pretty(&default_config)
            .context("Failed to serialize default config")?;
        fs::write("config.toml", default_content).context("Failed to write default config file")?;

        Ok(default_config)
    }

    /// Normalize a base URL by removing trailing slash
    fn normalize_base_url(url: &str) -> &str {
        url.trim_end_matches('/')
    }

    /// Join URL segments properly without creating double slashes
    pub fn join_url_segments(base: &str, segments: &[&str]) -> String {
        let normalized_base = Self::normalize_base_url(base);
        let mut url = normalized_base.to_string();
        
        for segment in segments {
            if !segment.is_empty() {
                url.push('/');
                url.push_str(segment);
            }
        }
        
        url
    }

    pub fn get_base_url(&self) -> &str {
        &self.webhook.base_url
    }

    pub fn get_body_preview_length(&self) -> usize {
        self.webhook.body_preview_length
    }
}
