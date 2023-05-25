mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use notify_run_rs_api::Notify;

/// NotifyRun Rust Client CLI entrypoint
fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Register(args) => {
            let notify = if let Some(api_server) = &args.api_server {
                Notify::register_from(api_server)
            } else {
                Notify::register()
            };

            match notify {
                Ok(notify) => {
                    println!("{}", notify);
                    Ok(())
                }
                Err(notify_err) => Err(format!("{}", notify_err)),
            }
        }
        Commands::Configure(args) => Ok(()),
        _ => Ok(()),
    }
}
