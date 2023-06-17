// TODO //!

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, name = "notify-run-rs")]
/// CLI application object
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
/// Possible CLI commands
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

#[derive(Args)]
/// Arguments for the configure command
pub(crate) struct ConfigureArgs {
    /// Notification endpoint URL
    pub(crate) endpoint: String,

    #[arg(short, long)]
    /// Don't prompt for config overwrite
    pub(crate) force: bool,
}

#[derive(Args)]
/// Arguments for the info command
pub(crate) struct InfoArgs {
    #[arg(short, long)]
    /// Notification endpoint URL
    pub(crate) endpoint: Option<String>,
}

#[derive(Args)]
/// Arguments for the register command
pub(crate) struct RegisterArgs {
    #[arg(short, long)]
    /// Set server URL, defaults to https://notify.run/api/ or the value of the NOTIFY_API_SERVER environment variable
    pub(crate) api_server: Option<String>,

    #[arg(short, long)]
    /// Don't prompt for config overwrite
    pub(crate) force: bool,
}

#[derive(Args)]
/// Arguments for the send command
pub(crate) struct SendArgs {
    /// The message text to be sent
    pub(crate) message: String,

    #[arg(short, long)]
    /// Notification endpoint URL
    pub(crate) endpoint: Option<String>,

    #[arg(short, long)]
    /// An optional URL to open if the notification is clicked.
    pub(crate) action: Option<String>,
}
