use anyhow::{Context, Result};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashSet;
use std::time::Duration;
use uuid::Uuid;

use crate::client::WebhookClient;
use crate::config::Config;
use crate::display::{
    print_full_request_body, print_request_body, print_request_details, print_request_headers,
    print_request_summary,
};

pub async fn generate_token(config: &Config) -> Result<()> {
    let token = Uuid::new_v4();
    let webhook_url = Config::join_url_segments(config.get_base_url(), &[&token.to_string()]);

    println!("{}", "New webhook token generated!".bright_green().bold());
    println!();
    println!(
        "{}: {}",
        "Token".bright_blue().bold(),
        token.to_string().bright_white()
    );
    println!(
        "{}: {}",
        "Webhook URL".bright_blue().bold(),
        webhook_url.bright_white()
    );
    println!();
    println!("{}", "Usage examples:".bright_yellow());
    println!("  webhook monitor --token {}", token);
    println!("  webhook logs --token {}", token);
    println!();

    Ok(())
}

pub async fn monitor_requests(
    client: &WebhookClient,
    token: &str,
    initial_count: u32,
    interval: u64,
    method_filter: Option<&str>,
    full_body: bool,
    show_headers: bool,
) -> Result<()> {
    println!("{}", "Starting webhook monitor...".bright_green().bold());
    println!("Token: {}", token.bright_white());
    if let Some(method) = method_filter {
        println!(
            "Filter: {} requests only",
            method.to_uppercase().bright_cyan()
        );
    }
    println!("Press {} to quit", "Ctrl+C".bright_red());
    println!("{}", "─".repeat(80).bright_black());

    let mut last_seen_ids = HashSet::new();
    let mut first_run = true;

    loop {
        match client.get_requests(token, initial_count).await {
            Ok(requests) => {
                let filtered_requests: Vec<_> = requests
                    .into_iter()
                    .filter(|req| {
                        method_filter.is_none_or(|method| {
                            req.message_object.method.to_lowercase() == method.to_lowercase()
                        })
                    })
                    .collect();

                if first_run {
                    // Show existing requests on first run
                    if filtered_requests.is_empty() {
                        println!(
                            "{}",
                            "No requests yet. Waiting for incoming webhooks...".bright_yellow()
                        );
                    } else {
                        println!(
                            "{} {} recent requests:",
                            "Found".bright_blue(),
                            filtered_requests.len()
                        );
                        for request in &filtered_requests {
                            print_request_summary(request);
                            if show_headers {
                                print_request_headers(request);
                            }
                            if full_body {
                                print_full_request_body(request);
                                println!(); // Add spacing between requests when showing full body
                            }
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
                        println!("{}", "NEW REQUEST".bright_green().bold());
                        print_request_summary(request);
                        if show_headers {
                            print_request_headers(request);
                        }
                        if full_body {
                            print_full_request_body(request);
                        } else {
                            print_request_body(request);
                        }
                        println!("{}", "─".repeat(80).bright_black());
                        last_seen_ids.insert(request.id.clone());
                    }
                }
            }
            Err(e) => {
                eprintln!("{} {}", "Error:".bright_red(), e);
            }
        }

        tokio::time::sleep(Duration::from_secs(interval)).await;
    }
}

pub async fn show_logs(
    client: &WebhookClient,
    token: &str,
    count: u32,
    method_filter: Option<&str>,
    full_body: bool,
    show_headers: bool,
) -> Result<()> {
    println!("{}", "Fetching webhook logs...".bright_blue().bold());

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner().template("{spinner} {msg}")?);
    spinner.set_message("Loading requests...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    let requests = client.get_requests(token, count).await?;
    spinner.finish_and_clear();

    let filtered_requests: Vec<_> = requests
        .into_iter()
        .filter(|req| {
            method_filter.is_none_or(|method| {
                req.message_object.method.to_lowercase() == method.to_lowercase()
            })
        })
        .collect();

    if filtered_requests.is_empty() {
        println!("{}", "No requests found.".bright_yellow());
        return Ok(());
    }

    println!(
        "{} {} requests for token {}",
        "Found".bright_blue(),
        filtered_requests.len(),
        token.bright_white()
    );

    if let Some(method) = method_filter {
        println!(
            "Filtered by method: {}",
            method.to_uppercase().bright_cyan()
        );
    }

    println!("{}", "─".repeat(80).bright_black());
    for request in &filtered_requests {
        print_request_summary(request);
        if show_headers {
            print_request_headers(request);
        }
        if full_body {
            print_full_request_body(request);
            println!(); // Add spacing between requests when showing full body
        }
    }

    println!();
    println!(
        "{}",
        "Use 'webhook show --token <token> --request-id <id>' for full details".bright_yellow()
    );

    Ok(())
}

pub async fn show_request_details(
    client: &WebhookClient,
    token: &str,
    request_id: &str,
) -> Result<()> {
    println!("{}", "Fetching request details...".bright_blue().bold());

    let requests = client.get_requests(token, 100).await?; // Get more requests to find the specific one

    let request = requests
        .into_iter()
        .find(|req| req.id == request_id)
        .with_context(|| format!("Request with ID {} not found", request_id))?;

    print_request_details(&request);

    Ok(())
}
