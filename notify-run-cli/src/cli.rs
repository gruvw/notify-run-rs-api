//! This module defines the CLI interface of the application. (using [clap])

use clap::{Args, Parser, Subcommand};

/// CLI application object
#[derive(Parser)]
#[command(author, version, about, name = "notify-run-rs")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

/// Possible CLI commands
#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Configure to an existing endpoint
    Configure(ConfigureArgs),

    /// Get information for subscribing to an endpoint and lists the previous messages
    Info(InfoArgs),

    /// Register a new endpoint
    Register(RegisterArgs),

    /// Send a new notification message
    Send(SendArgs),
}

/// Arguments for the configure command
#[derive(Args)]
pub(crate) struct ConfigureArgs {
    /// Notification endpoint URL
    pub(crate) endpoint: String,

    /// Don't prompt for config overwrite
    #[arg(short, long)]
    pub(crate) force: bool,
}

/// Arguments for the info command
#[derive(Args)]
pub(crate) struct InfoArgs {
    /// Notification endpoint URL
    #[arg(short, long)]
    pub(crate) endpoint: Option<String>,
}

/// Arguments for the register command
#[derive(Args)]
pub(crate) struct RegisterArgs {
    /// Set server URL, defaults to https://notify.run/api/ or the value of the NOTIFY_API_SERVER environment variable
    #[arg(short, long)]
    pub(crate) api_server: Option<String>,

    /// Don't prompt for config overwrite
    #[arg(short, long)]
    pub(crate) force: bool,
}

/// Arguments for the send command
#[derive(Args)]
pub(crate) struct SendArgs {
    /// The message text to be sent
    pub(crate) message: String,

    /// Notification endpoint URL
    #[arg(short, long)]
    pub(crate) endpoint: Option<String>,

    /// An optional URL to open if the notification is clicked.
    #[arg(short, long)]
    pub(crate) action: Option<String>,
}
