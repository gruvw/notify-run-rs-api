mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use inquire::Confirm;
use notify_run_rs_api::Notify;

/// NotifyRun Rust Client CLI entrypoint
fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Register(args) => {
            if let Ok(notify) = Notify::from_config() {
                if !args.force {
                    let ans = Confirm::new(&format!(
                        "Overwrite existing configuration ({})?",
                        notify.endpoint()
                    ))
                    .with_default(false)
                    .prompt();

                    match ans {
                        Ok(true) => (),
                        Ok(false) => return Ok(()),
                        Err(_) => return Err("Error with prompt, try again later.".to_string()),
                    }
                }
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
        Commands::Configure(args) => Ok(()),
        _ => Ok(()),
    }
}
