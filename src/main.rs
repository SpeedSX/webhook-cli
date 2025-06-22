use anyhow::{Context, Result};
use chrono::{DateTime};
use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "webhook")]
#[command(about = "A CLI tool for webhook testing and monitoring")]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new webhook token
    Generate,
    /// Monitor webhook requests in real-time
    Monitor {
        /// Webhook token (GUID)
        #[arg(short, long)]
        token: Option<String>,
        /// Number of recent requests to show initially
        #[arg(short, long, default_value = "10")]
        count: u32,
        /// Refresh interval in seconds
        #[arg(short, long, default_value = "3")]
        interval: u64,
        /// Show only specific HTTP method
        #[arg(short, long)]
        method: Option<String>,
    },
    /// Show request logs for a token
    Logs {
        /// Webhook token (GUID)
        #[arg(short, long)]
        token: String,
        /// Number of requests to fetch
        #[arg(short, long, default_value = "50")]
        count: u32,
        /// Show only specific HTTP method
        #[arg(short, long)]
        method: Option<String>,
    },
    /// Show details of a specific request
    Show {
        /// Webhook token (GUID)
        #[arg(short, long)]
        token: String,
        /// Request ID to show details for
        #[arg(short, long)]
        request_id: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
struct WebhookRequest {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "TokenId")]
    token_id: String,
    #[serde(rename = "MessageObject")]
    message_object: MessageObject,
    #[serde(rename = "Message")]
    message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MessageObject {
    #[serde(rename = "Method")]
    method: String,
    #[serde(rename = "Value")]
    value: String,
    #[serde(rename = "Headers")]
    headers: HashMap<String, Vec<String>>,
    #[serde(rename = "QueryParameters")]
    query_parameters: Vec<String>,
    #[serde(rename = "Body")]
    body: Option<String>,
    #[serde(rename = "BodyObject")]
    body_object: Option<serde_json::Value>,
}

struct WebhookClient {
    client: Client,
    base_url: String,
}

impl WebhookClient {
    fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://webhooktest.emergemarket.dev".to_string(),
        }
    }

    async fn get_requests(&self, token: &str, count: u32) -> Result<Vec<WebhookRequest>> {
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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = WebhookClient::new();

    match cli.command {
        Commands::Generate => {
            let token = Uuid::new_v4();
            let webhook_url = format!("https://webhooktest.emergemarket.dev/{}", token);
            
            println!("{}", "🔑 New webhook token generated!".bright_green().bold());
            println!();
            println!("{}: {}", "Token".bright_blue().bold(), token.to_string().bright_white());
            println!("{}: {}", "Webhook URL".bright_blue().bold(), webhook_url.bright_white());
            println!();
            println!("{}", "💡 Usage examples:".bright_yellow());
            println!("  webhook monitor --token {}", token);
            println!("  webhook logs --token {}", token);
            println!();
        }

        Commands::Monitor { token, count, interval, method } => {
            let token = match token {
                Some(t) => t,
                None => {
                    // Generate a new token if none provided
                    let new_token = Uuid::new_v4();
                    println!("{}", "🔑 No token provided, generated a new one:".bright_yellow());
                    println!("{}: {}", "Token".bright_blue().bold(), new_token.to_string().bright_white());
                    println!("{}: https://webhooktest.emergemarket.dev/{}", "Webhook URL".bright_blue().bold(), new_token.to_string().bright_white());
                    println!();
                    new_token.to_string()
                }
            };

            monitor_requests(&client, &token, count, interval, method.as_deref()).await?;
        }

        Commands::Logs { token, count, method } => {
            show_logs(&client, &token, count, method.as_deref()).await?;
        }

        Commands::Show { token, request_id } => {
            show_request_details(&client, &token, &request_id).await?;
        }
    }

    Ok(())
}

async fn monitor_requests(
    client: &WebhookClient,
    token: &str,
    initial_count: u32,
    interval: u64,
    method_filter: Option<&str>,
) -> Result<()> {
    println!("{}", "🔍 Starting webhook monitor...".bright_green().bold());
    println!("Token: {}", token.bright_white());
    if let Some(method) = method_filter {
        println!("Filter: {} requests only", method.to_uppercase().bright_cyan());
    }
    println!("Press {} to quit", "Ctrl+C".bright_red());
    println!("{}", "─".repeat(80).bright_black());

    let mut last_seen_ids = std::collections::HashSet::new();
    let mut first_run = true;

    loop {
        match client.get_requests(token, initial_count).await {
            Ok(requests) => {
                let filtered_requests: Vec<_> = requests
                    .into_iter()
                    .filter(|req| {
                        method_filter.map_or(true, |method| {
                            req.message_object.method.to_lowercase() == method.to_lowercase()
                        })
                    })
                    .collect();

                if first_run {
                    // Show existing requests on first run
                    if filtered_requests.is_empty() {
                        println!("{}", "📭 No requests yet. Waiting for incoming webhooks...".bright_yellow());
                    } else {
                        println!("{} {} recent requests:", "📜".bright_blue(), filtered_requests.len());
                        for request in &filtered_requests {
                            print_request_summary(request);
                            last_seen_ids.insert(request.id.clone());
                        }
                    }
                    first_run = false;
                } else {
                    // Show only new requests
                    let new_requests: Vec<_> = filtered_requests
                        .into_iter()
                        .filter(|req| !last_seen_ids.contains(&req.id))
                        .collect();

                    for request in &new_requests {
                        println!("{}", "🆕 NEW REQUEST".bright_green().bold());
                        print_request_summary(request);
                        print_request_body(request);
                        println!("{}", "─".repeat(80).bright_black());
                        last_seen_ids.insert(request.id.clone());
                    }
                }
            }
            Err(e) => {
                eprintln!("{} {}", "❌ Error:".bright_red(), e);
            }
        }

        tokio::time::sleep(Duration::from_secs(interval)).await;
    }
}

async fn show_logs(
    client: &WebhookClient,
    token: &str,
    count: u32,
    method_filter: Option<&str>,
) -> Result<()> {
    println!("{}", "📜 Fetching webhook logs...".bright_blue().bold());
    
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner().template("{spinner} {msg}")?);
    spinner.set_message("Loading requests...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    let requests = client.get_requests(token, count).await?;
    spinner.finish_and_clear();

    let filtered_requests: Vec<_> = requests
        .into_iter()
        .filter(|req| {
            method_filter.map_or(true, |method| {
                req.message_object.method.to_lowercase() == method.to_lowercase()
            })
        })
        .collect();

    if filtered_requests.is_empty() {
        println!("{}", "📭 No requests found.".bright_yellow());
        return Ok(());
    }

    println!("{} {} requests for token {}", 
        "📊".bright_blue(), 
        filtered_requests.len(), 
        token.bright_white()
    );
    
    if let Some(method) = method_filter {
        println!("Filtered by method: {}", method.to_uppercase().bright_cyan());
    }
    
    println!("{}", "─".repeat(80).bright_black());

    for request in &filtered_requests {
        print_request_summary(request);
    }

    println!();
    println!("{}", "💡 Use 'webhook show --token <token> --request-id <id>' for full details".bright_yellow());

    Ok(())
}

async fn show_request_details(
    client: &WebhookClient,
    token: &str,
    request_id: &str,
) -> Result<()> {
    println!("{}", "🔍 Fetching request details...".bright_blue().bold());
    
    let requests = client.get_requests(token, 100).await?; // Get more requests to find the specific one
    
    let request = requests
        .into_iter()
        .find(|req| req.id == request_id)
        .with_context(|| format!("Request with ID {} not found", request_id))?;

    println!("{}", "📋 REQUEST DETAILS".bright_green().bold());
    println!("{}", "═".repeat(50).bright_black());
    
    // Basic info
    println!("{}: {}", "ID".bright_blue().bold(), request.id.bright_white());
    println!("{}: {}", "Token".bright_blue().bold(), request.token_id.bright_white());
    println!("{}: {}", "Date".bright_blue().bold(), format_date(&request.date).bright_white());
    println!("{}: {}", "Method".bright_blue().bold(), format_method(&request.message_object.method));
    println!("{}: {}", "Path".bright_blue().bold(), request.message_object.value.bright_white());
    println!();

    // Headers
    println!("{}", "📋 HEADERS".bright_cyan().bold());
    println!("{}", "─".repeat(30).bright_black());
    for (key, values) in &request.message_object.headers {
        for value in values {
            println!("{}: {}", key.bright_blue(), value.bright_white());
        }
    }
    println!();

    // Query Parameters
    if !request.message_object.query_parameters.is_empty() {
        println!("{}", "🔍 QUERY PARAMETERS".bright_cyan().bold());
        println!("{}", "─".repeat(30).bright_black());
        for param in &request.message_object.query_parameters {
            println!("{}", param.bright_white());
        }
        println!();
    }

    // Body
    println!("{}", "📄 REQUEST BODY".bright_cyan().bold());
    println!("{}", "─".repeat(30).bright_black());
    if let Some(body) = &request.message_object.body {
        if body.trim().is_empty() {
            println!("{}", "(empty)".bright_black());
        } else {
            // Try to pretty-print JSON
            match serde_json::from_str::<serde_json::Value>(body) {
                Ok(json) => {
                    println!("{}", serde_json::to_string_pretty(&json).unwrap().bright_white());
                }
                Err(_) => {
                    println!("{}", body.bright_white());
                }
            }
        }
    } else {
        println!("{}", "(no body)".bright_black());
    }

    Ok(())
}

fn print_request_summary(request: &WebhookRequest) {
    let time = format_date(&request.date);
    let method = format_method(&request.message_object.method);
    let path = extract_path(&request.message_object.value, &request.token_id);
    let id_short = &request.id[..8];

    println!(
        "{} {} {} {} {}",
        time.bright_black(),
        method,
        path.bright_white(),
        format!("({}...)", id_short).bright_black(),
        get_body_preview(&request.message_object.body).bright_yellow()
    );
}

fn print_request_body(request: &WebhookRequest) {
    if let Some(body) = &request.message_object.body {
        if !body.trim().is_empty() {
            println!("{}: {}", "Body".bright_blue().bold(), 
                if body.len() > 200 {
                    format!("{}...", &body[..200])
                } else {
                    body.clone()
                }.bright_white()
            );
        }
    }
}

fn format_method(method: &str) -> colored::ColoredString {
    match method.to_uppercase().as_str() {
        "GET" => method.green().bold(),
        "POST" => method.blue().bold(),
        "PUT" => method.yellow().bold(),
        "DELETE" => method.red().bold(),
        "PATCH" => method.magenta().bold(),
        _ => method.white().bold(),
    }
}

fn format_date(date_str: &str) -> String {
    match DateTime::parse_from_rfc3339(date_str) {
        Ok(dt) => dt.format("%H:%M:%S").to_string(),
        Err(_) => date_str.to_string(),
    }
}

fn extract_path(full_path: &str, token: &str) -> String {
    if let Some(token_index) = full_path.find(token) {
        let after_token = &full_path[token_index + token.len()..];
        if after_token.is_empty() {
            "/".to_string()
        } else {
            after_token.to_string()
        }
    } else {
        full_path.to_string()
    }
}

fn get_body_preview(body: &Option<String>) -> String {
    match body {
        Some(b) if !b.trim().is_empty() => {
            if b.len() > 50 {
                format!("📄 {}", &b[..47].trim())
            } else {
                format!("📄 {}", b.trim())
            }
        }
        _ => "📄 (empty)".to_string(),
    }
}
