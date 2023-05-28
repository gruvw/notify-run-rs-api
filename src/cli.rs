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
}

#[derive(Args)]
pub struct ConfigureArgs {
    /// Notification endpoint URL
    pub endpoint: String,

    #[arg(short, long)]
    /// Don't prompt for config overwrite
    force: bool,
}

#[derive(Args)]
pub struct InfoArgs {
    /// Notification endpoint URL
    pub endpoint: String,
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
