use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use uuid::Uuid;

mod cli;
mod client;
mod color_control;
mod commands;
mod config;
mod display;
mod models;

use cli::{Cli, Commands};
use client::WebhookClient;
use commands::{generate_token, monitor_requests, show_logs, show_request_details};
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize color control
    let no_color_env = std::env::var_os("NO_COLOR").is_some();
    color_control::init(cli.no_color || no_color_env);

    let config = Config::load()?;
    let client = WebhookClient::new(&config);

    match cli.command {
        Commands::Generate => {
            generate_token(&config).await?;
        }

        Commands::Monitor {
            token,
            count,
            interval,
            method,
            full_body,
            show_headers,
            parse,
        } => {
            let token = match token {
                Some(t) => t,
                None => {
                    // Generate a new token if none provided
                    let new_token = Uuid::new_v4();
                    println!(
                        "{}",
                        "No token provided, generated a new one:".bright_yellow()
                    );
                    println!(
                        "{}: {}",
                        "Token".bright_blue().bold(),
                        new_token.to_string().bright_white()
                    );
                    println!(
                        "{}: {}/{}",
                        "Webhook URL".bright_blue().bold(),
                        config.get_base_url(),
                        new_token.to_string().bright_white()
                    );
                    println!();
                    new_token.to_string()
                }
            };

            monitor_requests(
                &client,
                &config,
                &token,
                count,
                interval,
                method.as_deref(),
                full_body,
                show_headers,
                &parse,
            )
            .await?;
        }
        Commands::Logs {
            token,
            count,
            method,
            full_body,
            show_headers,
            parse,
        } => {
            show_logs(
                &client,
                &config,
                &token,
                count,
                method.as_deref(),
                full_body,
                show_headers,
                &parse,
            )
            .await?;
        }

        Commands::Show {
            token,
            request_id,
            parse,
        } => {
            show_request_details(&client, &token, &request_id, &parse).await?;
        }
    }

    Ok(())
}
