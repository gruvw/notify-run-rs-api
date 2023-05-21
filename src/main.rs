mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use qrcode::{render::unicode, QrCode};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::QRDisplay(site) => {
            let code = QrCode::new(&site.url).unwrap();
            let image = code
                .render::<unicode::Dense1x2>()
                .dark_color(unicode::Dense1x2::Light)
                .light_color(unicode::Dense1x2::Dark)
                .build();
            println!("{}", image);
        }
    }
}
