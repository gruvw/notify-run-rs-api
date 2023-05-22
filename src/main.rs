mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use notify_run_rs_api::Notify;
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Register(args) => {
            let notify = Notify::register().unwrap();
            println!("{}", notify);
        }
        _ => {}
    }
}
