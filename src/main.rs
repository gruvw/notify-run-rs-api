mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use inquire::Confirm;
use notify_run_rs_api::Notify;

/// Returns true if the config should be overwritten, else otherwise
fn should_write(force: bool) -> Result<bool, String> {
    if !force {
        if let Ok(notify) = Notify::from_config() {
            let ans = Confirm::new(&format!(
                "Overwrite existing configuration ({})?",
                notify.endpoint()
            ))
            .with_default(false)
            .prompt();

            return match ans {
                Ok(res) => Ok(res),
                Err(_) => Err("Error with prompt, try again later.".to_string()),
            };
        }
    }

    Ok(true)
}

/// NotifyRun Rust Client CLI entrypoint
fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Register(args) => {
            if !should_write(args.force)? {
                return Ok(());
            }

            let notify = if let Some(api_server) = &args.api_server {
                Notify::register_from(api_server.to_string())
            } else {
                Notify::register()
            };

            match notify {
                Ok(notify) => {
                    notify.update_config().map_err(|e| format!("{}", e))?;
                    println!("{}", notify);
                    Ok(())
                }
                Err(notify_err) => Err(format!("{}", notify_err)),
            }
        }

        Commands::Configure(args) => {
            let notify = Notify::from_endpoint(&args.endpoint).map_err(|e| format!("{}", e))?;

            if should_write(args.force)? {
                notify.update_config().map_err(|e| format!("{}", e))?;
            }

            Ok(())
        }

        _ => Ok(()),
    }
}
