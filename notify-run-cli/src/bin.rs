//! The notify.run Rust Client entrypoint module.

mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use inquire::Confirm;
use notify_run::Notify;

/// notify.run Rust Client CLI entrypoint
fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Register(args) => {
            if !should_write(args.force)? {
                return Ok(());
            }

            let notify = if let Some(api_server) = &args.api_server {
                Notify::register_from(api_server.into())
            } else {
                Notify::register()
            };

            match notify {
                Ok(notify) => {
                    notify.write_to_config().map_err(|e| e.to_string())?;
                    println!("{}", notify);
                    Ok(())
                }
                Err(notify_err) => Err(notify_err.to_string()),
            }
        }

        Commands::Configure(args) => {
            let notify = Notify::from_endpoint(&args.endpoint).map_err(|e| e.to_string())?;

            if should_write(args.force)? {
                notify.write_to_config().map_err(|e| e.to_string())?;
            }

            Ok(())
        }

        Commands::Send(args) => {
            let notify = get_notify_instance(&args.endpoint)?;

            if let Some(action) = &args.action {
                notify.send_action(&args.message, action)
            } else {
                notify.send(&args.message)
            }
            .map_err(|e| e.to_string())
        }

        Commands::Info(args) => {
            let notify = get_notify_instance(&args.endpoint)?;
            let messages = notify.messages().map_err(|e| e.to_string())?;

            println!("{}", notify);
            println!("Messages history:");

            if messages.is_empty() {
                println!("No messages.")
            }
            for message in messages {
                println!("{}", message);
            }

            Ok(())
        }
    }
}

/// Returns true if the config should be overwritten, false otherwise
fn should_write(force: bool) -> Result<bool, String> {
    if !force {
        if let Ok(notify) = Notify::from_config() {
            let ans = Confirm::new(
                format!("Overwrite existing configuration ({})?", notify.endpoint()).as_str(),
            )
            .with_default(false)
            .prompt();

            return match ans {
                Ok(res) => Ok(res),
                Err(_) => Err("Error with prompt, try again later.".into()),
            };
        }
    }

    Ok(true)
}

/// Returns the Notify instance.
/// Tries to get it from passed endpoint first, from config otherwise.
fn get_notify_instance(endpoint: &Option<String>) -> Result<Notify, String> {
    if let Some(endpoint) = endpoint {
        Ok(Notify::from_endpoint(endpoint).map_err(|e| e.to_string())?)
    } else if let Ok(notify) = Notify::from_config() {
        Ok(notify)
    } else {
        Err(
            "No endpoint found! Run 'register' or 'configure' first. See help for more details."
                .into(),
        )
    }
}
