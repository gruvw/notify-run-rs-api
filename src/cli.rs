use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Configure to an existing endpoint
    Configure(ConfigureArgs),

    /// Get information for subscribing to an endpoint
    Info(InfoArgs),

    /// Register a new endpoint
    Register(RegisterArgs),

    // Send a new message
    Send(SendArgs),
}

#[derive(Args)]
pub struct ConfigureArgs {
    /// Notification endpoint URL
    pub endpoint: String,

    #[arg(short, long)]
    /// Don't prompt for config overwrite
    pub force: bool,
}

#[derive(Args)]
pub struct InfoArgs {
    #[arg(short, long)]
    /// Notification endpoint URL
    pub endpoint: Option<String>,
}

#[derive(Args)]
pub struct RegisterArgs {
    #[arg(short, long)]
    /// Set server URL, defaults to https://notify.run/api/ or the value of the NOTIFY_API_SERVER environment variable
    pub api_server: Option<String>,

    #[arg(short, long)]
    /// Don't prompt for config overwrite
    pub force: bool,
}

#[derive(Args)]
pub struct SendArgs {
    /// The message text to be sent.
    pub message: String,

    #[arg(short, long)]
    /// Notification endpoint URL
    pub endpoint: Option<String>,

    #[arg(short, long)]
    /// An optional URL to open if the notification is clicked.
    pub action: Option<String>,
}
