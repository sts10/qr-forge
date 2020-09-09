extern crate structopt;
use qrforge::*;
use std::path::PathBuf;
use structopt::StructOpt;

/// QR Forge
#[derive(StructOpt, Debug)]
#[structopt(name = "qrforge")]
enum Opt {
    /// Encode a QR code from text secret, service, and username
    #[structopt(alias = "encode")]
    Encode,

    /// Decode a QR code image file to an OTPauth URI
    #[structopt(alias = "decode")]
    Decode {
        #[structopt(name = "QR image", parse(from_os_str))]
        qr_image_file: PathBuf,
    },
}

fn main() {
    match Opt::from_args() {
        Opt::Encode => encode_qr_code(),
        Opt::Decode { qr_image_file } => decode_qr_code(qr_image_file),
    }
}
