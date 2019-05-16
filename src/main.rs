extern crate structopt;
use qrforge::*;
use std::path::PathBuf;
use structopt::StructOpt;

/// QR Forge
#[derive(StructOpt, Debug)]
#[structopt(name = "qrforge")]
struct Opt {
    /// Encode QR code from text secret, service and username
    #[structopt(short = "e", long = "encode")]
    encode: bool,

    /// Decode a QR code image file to an OTPauth URI
    #[structopt(short = "d", long = "decode", parse(from_os_str))]
    qr_image_file: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    match opt.qr_image_file {
        Some(qr_image_file) => match read_codes_from_file(&qr_image_file) {
            Ok(codes) => {
                println!("Discovered {} code(s):", codes.len());
                for code in codes {
                    println!("{}", code);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        None => encode_qr_code(),
    }
}
