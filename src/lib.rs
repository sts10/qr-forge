extern crate qrcode;
extern crate rpassword;
use byteorder::{BigEndian, ByteOrder};
use chrono::Local;
use crypto::mac::Mac;
use crypto::{hmac::Hmac, sha1::Sha1};
use data_encoding::BASE32;
use qrcode::QrCode;
use std::error::Error;
use std::io::{self, ErrorKind};

pub fn get_key() -> String {
    rpassword::read_password_from_tty(Some("Enter the secret key:\n"))
        .expect("Error reading secret key")
        .chars()
        .filter(|&c| c != ' ')
        .collect()
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

pub fn present_codes(key: &str) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    for token_number in 0..7 {
        tokens.push(generate_otp_token(key, token_number * 30).expect("Problem generating token"));
    }
    tokens
}
fn generate_otp_token(key: &str, token_number: i64) -> Result<String, Box<Error>> {
    let now = Local::now().timestamp();
    let timer = ((now + token_number) / 30) as u64;
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

pub fn gets(prompt: &str) -> io::Result<String> {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => Ok(input.trim_end_matches('\n').to_string()),
        Err(error) => Err(error),
    }
}
