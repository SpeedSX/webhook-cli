use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "webhook")]
#[command(about = "A CLI tool for webhook testing and monitoring")]
#[command(version)]
pub struct Cli {
    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
        /// Show full request body with proper formatting
        #[arg(long)]
        full_body: bool,
        /// Show request headers
        #[arg(long)]
        show_headers: bool,
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
        /// Show full request body with proper formatting
        #[arg(long)]
        full_body: bool,
        /// Show request headers
        #[arg(long)]
        show_headers: bool,
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