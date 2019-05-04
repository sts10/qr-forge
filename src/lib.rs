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

pub fn display_qr_code(otpauth_uri: &str) -> Result<(), io::Error> {
    let code = QrCode::new(otpauth_uri).unwrap();

    let string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("{}", string);
    Ok(())
}

pub fn present_codes(key: &str) -> Result<Vec<String>, Box<Error>> {
    let mut tokens: Vec<String> = vec![];
    for token_number in 0..7 {
        match generate_otp_token(key, token_number * 30) {
            Ok(this_token) => tokens.push(this_token),
            Err(e) => return Err(e),
        }
    }
    Ok(tokens)
}
fn test_key(key: &str) -> Result<(), Box<Error>> {
    let secret_bytes = BASE32.decode(key.as_bytes());
    match secret_bytes {
        Ok(_bytes) => Ok(()),
        Err(_) => Err(io::Error::new(
            ErrorKind::InvalidInput,
            "key is not a valid base32 data type",
        )
        .into()),
    }
}

fn generate_otp_token(key: &str, future_seconds: i64) -> Result<String, Box<Error>> {
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

pub fn make_qr_code_image(otpauth_uri: &str) -> Result<&str, io::Error> {
    let code = QrCode::new(otpauth_uri).unwrap();
    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();
    // Save the image.
    let qr_code_file_path = "qr-code.png";
    image.save(qr_code_file_path).unwrap();
    Ok(qr_code_file_path)
}
pub fn gets(prompt: &str) -> io::Result<String> {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => Ok(input.trim_end_matches('\n').to_string()),
        Err(error) => Err(error),
    }
}
