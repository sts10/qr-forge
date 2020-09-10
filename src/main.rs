extern crate structopt;
use qrforge::*;
use std::path::PathBuf;
use structopt::StructOpt;

/// QR Forge
#[derive(StructOpt, Debug)]
#[structopt(name = "qrforge")]
enum Opt {
    /// Draw a QR code from text secret, service, and username
    #[structopt(alias = "draw")]
    Draw,

    /// Read a QR code image file to an OTPauth URI
    #[structopt(alias = "read")]
    Read {
        #[structopt(name = "QR image", parse(from_os_str))]
        qr_image_file: PathBuf,
    },
}

fn main() {
    match Opt::from_args() {
        Opt::Draw => draw_qr_code(),
        Opt::Read { qr_image_file } => read_qr_code(qr_image_file),
    }
}
