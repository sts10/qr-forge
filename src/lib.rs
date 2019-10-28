extern crate image;
extern crate qrcode;
extern crate rpassword;
use byteorder::{BigEndian, ByteOrder};
use chrono::Local;
use crypto::mac::Mac;
use crypto::{hmac::Hmac, sha1::Sha1};
use data_encoding::BASE32;
use image::Luma;
use qrcode::QrCode;
use std::error::Error;
use std::io::{self, ErrorKind};
use std::path::PathBuf;
extern crate quirc;

use quirc::QrCoder;
use std::fs::File;
use std::io::Read;
use std::str;

pub fn encode_qr_code() {
    println!("Let's make a QR code");
    let key: String = get_key();
    // MVTGOZDHMRTGOZDGM5QWOZ3BM5TWOZ3H
    let service = gets("Enter name of service").unwrap();
    let username = gets("Enter username").unwrap();

    let otpauth_uri = make_otpauth_uri(&key, service, username);

    display_qr_code(&otpauth_uri).expect("Couldn't display QR code");

    match present_codes(&key) {
        Ok(codes) => println!("Next couple of tokens: {:?}", codes),
        Err(e) => eprintln!("Error: {}", e),
    }
    let image_decision =
        gets("Would you like to create an image file of this QR code? (y/N)").unwrap();
    if image_decision == "y" {
        match make_qr_code_image(&otpauth_uri) {
            Ok(file_path) => println!(
                "QR image file generated at {}. Be sure to delete it securely when done.",
                file_path
            ),
            Err(e) => eprintln!("Error generating QR code image file: {}", e),
        }
    } else {
        println!("OK, I won't create a file. All done");
    }
}

pub fn get_key() -> String {
    loop {
        // get key from user safely, then make it uppercase and remove spaces
        let key: String = rpassword::read_password_from_tty(Some("Enter the secret key:\n"))
            .expect("Error reading secret key")
            .to_ascii_uppercase()
            .chars()
            .filter(|&c| c != ' ')
            .collect();
        // test key for validity. If invalid, start loop again
        match test_key(&key) {
            Ok(()) => return key,
            Err(e) => {
                eprintln!(
                    "Key is not 32 characters or is otherwise invalid ({}).\nTry again.",
                    e
                );
                continue;
            }
        }
    }
}

pub fn make_otpauth_uri(key: &str, service: String, username: String) -> String {
    format!(
        "otpauth://totp/{}:@{}?secret={}&issuer={}",
        service, username, key, service
    )
}

pub fn display_qr_code(otpauth_uri: &str) -> Result<(), qrcode::types::QrError> {
    let code = QrCode::new(otpauth_uri)?;

    let string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("{}", string);
    Ok(())
}

pub fn present_codes(key: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut tokens: Vec<String> = vec![];
    for token_number in 0..7 {
        match generate_otp_token(key, token_number * 30) {
            Ok(this_token) => tokens.push(this_token),
            Err(e) => return Err(e),
        }
    }
    Ok(tokens)
}
fn test_key(key: &str) -> Result<(), Box<dyn Error>> {
    let secret_bytes = BASE32.decode(key.as_bytes());
    match secret_bytes {
        Ok(_bytes) => Ok(()),
        Err(_) => Err(io::Error::new(
            ErrorKind::InvalidInput,
            "key is not a valid BASE32 data type",
        )
        .into()),
    }
}

fn generate_otp_token(key: &str, future_seconds: i64) -> Result<String, Box<dyn Error>> {
    let now = Local::now().timestamp();
    let timer = ((now + future_seconds) / 30) as u64;
    let secret_bytes = BASE32.decode(key.as_bytes());
    let bytes = match secret_bytes {
        Ok(bytes) => bytes,
        Err(_) => {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "key is not a valid base32 data type",
            )
            .into());
        }
    };
    let mut buf = [0; 8];
    let mut hm = Hmac::new(Sha1::new(), &bytes[..]);
    BigEndian::write_u64(&mut buf, timer);
    hm.input(&buf[..]);
    let res = hm.result();
    let result = res.code();
    let offset = match &result.last() {
        Some(l) => *l & 0xf,
        None => {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "was not able to get last byte of hmac result",
            )
            .into());
        }
    };
    let offset = offset as usize;
    let value = i64::from(
        ((i32::from(result[offset]) & 0x7f) << 24)
            | ((i32::from(result[offset + 1]) & 0xff) << 16)
            | ((i32::from(result[offset + 2]) & 0xff) << 8)
            | (i32::from(result[offset + 3]) & 0xff),
    );

    let length = 6;
    let pow10: i64 = 10;
    let modulo = value % pow10.pow(length);
    Ok(format!("{:0length$}", modulo, length = 6))
}

// pub fn make_qr_code_image(otpauth_uri: &str) -> Result<&str, qrcode::types::QrError> {
pub fn make_qr_code_image(otpauth_uri: &str) -> Result<&str, Box<dyn Error>> {
    let code = QrCode::new(otpauth_uri)?;
    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();
    // Save the image.
    let qr_code_file_path = "qr-code.png";
    match image.save(qr_code_file_path) {
        Ok(_) => (),
        Err(_) => {
            return Err(
                io::Error::new(ErrorKind::InvalidInput, "Error saving QR code image file").into(),
            );
        }
    }
    Ok(qr_code_file_path)
}

pub fn read_codes_from_file(file_path: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut vec = Vec::new();
    match file.read_to_end(&mut vec) {
        Ok(_) => (),
        Err(_) => {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "Error reading QR code image file",
            )
            .into());
        }
    }

    let image = image::load_from_memory(&vec).unwrap().to_luma();

    let mut quirc = match QrCoder::new() {
        Ok(code) => code,
        Err(_) => {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "Error reading QR code image file",
            )
            .into());
        }
    };

    let width = image.width();
    let height = image.height();
    let codes = match quirc.codes(&image, width, height) {
        Ok(codes) => codes,
        Err(_) => {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "Error reading QR code image file",
            )
            .into());
        }
    };
    let mut codes_as_strings: Vec<String> = vec![];

    for code in codes {
        match code {
            Ok(code) => codes_as_strings.push(qrcode_to_string(code)),
            Err(_) => {
                return Err(io::Error::new(
                    ErrorKind::InvalidData,
                    "Error reading data from QR code image file",
                )
                .into());
            }
        }
    }
    Ok(codes_as_strings)
}

fn qrcode_to_string(code: quirc::QrCode) -> String {
    String::from(str::from_utf8(&code.payload).expect("Error reading QR image payload"))
}

pub fn gets(prompt: &str) -> io::Result<String> {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => Ok(input.trim_end_matches('\n').to_string()),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    #[test]
    fn can_write_and_read_a_qr_code_image_file() {
        let key: String = "secretkeytest".to_string();
        let service = "MySocialNetwork".to_string();
        let username = "test_user".to_string();

        let otpauth_uri = make_otpauth_uri(&key, service, username);

        let qr_image_file_path = match make_qr_code_image(&otpauth_uri) {
            Ok(file_path) => file_path,

            Err(e) => panic!("Error generating QR code image file: {}", e),
        };

        let first_code =
            &read_codes_from_file(&Path::new(qr_image_file_path).to_path_buf()).unwrap()[0];
        assert_eq!(first_code, &otpauth_uri);
    }
}
