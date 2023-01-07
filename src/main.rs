use clap::{Parser, Subcommand};
use qrforge::*;
use std::path::PathBuf;

/// Safely transform between 32-character TOTP secret keys and their QR codes
#[derive(Parser)]
#[clap(version, name = "qrforge")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Draw a QR code from given TOTP text secret, service, and username
    #[clap(alias = "draw")]
    Draw {
        /// Print created QR code to a file. I'd suggest using a .png file
        /// extension.
        #[clap(short = 'o', long = "output")]
        output: Option<PathBuf>,
    },

    /// Read a QR code image file and prints OTPauth URI
    #[clap(alias = "read")]
    Read {
        /// File path of QR image to read
        #[clap(name = "QR image")]
        qr_image_file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Draw { output } => draw_qr_code(output),
        Commands::Read { qr_image_file } => read_qr_code(&qr_image_file),
    }
}
