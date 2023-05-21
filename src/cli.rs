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
    /// Displays website as QR code
    QRDisplay(SiteArgs),
}

#[derive(Args)]
pub struct SiteArgs {
    pub url: String,
}
