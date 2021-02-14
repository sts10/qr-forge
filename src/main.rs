extern crate structopt;
use qrforge::*;
use std::path::PathBuf;
use structopt::StructOpt;

/// QR Forge
#[derive(StructOpt, Debug)]
#[structopt(name = "qrforge")]
enum Opt {
    /// Draw a QR code from TOTP text secret, service, and username
    #[structopt(alias = "draw")]
    Draw {
        /// Print created QR code to a file
        #[structopt(short = "o", long = "output")]
        output: Option<String>,
    },

    /// Read a QR code image file and prints OTPauth URI
    #[structopt(alias = "read")]
    Read {
        /// File path of QR image to read
        #[structopt(name = "QR image", parse(from_os_str))]
        qr_image_file: PathBuf,
    },
}

fn main() {
    match Opt::from_args() {
        Opt::Draw { output } => draw_qr_code(output),
        Opt::Read { qr_image_file } => read_qr_code(qr_image_file),
    }
}
