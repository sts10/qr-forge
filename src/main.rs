use qrforge::*;
use std::path::PathBuf;
use clap::Parser;

/// Safely handle TOTP secrets and their QR codes
#[derive(Parser, Debug)]
#[clap(version, name = "qrforge")]
struct Args {
    /// Draw a QR code from TOTP text secret, service, and username
    #[clap(alias = "draw")]
    Draw: {
        /// Print created QR code to a file
        #[clap(short = "o", long = "output")]
        output: Option<String>,
    },

    /// Read a QR code image file and prints OTPauth URI
    #[clap(alias = "read")]
    Read: {
        /// File path of QR image to read
        #[clap(name = "QR image", parse(from_os_str))]
        qr_image_file: PathBuf,
    },
}

fn main() {
    match Args::parse() {
        Args::Draw { output } => draw_qr_code(output),
        Args::Read { qr_image_file } => read_qr_code(&qr_image_file),
    }
}
