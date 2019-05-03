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

fn main() {
    let key: String = rpassword::read_password_from_tty(Some("Enter the secret key:\n"))
        .expect("Error reading secret key")
        .chars()
        .filter(|&c| c != ' ')
        .collect();

    println!("Enter name of service:");
    let service = gets().unwrap();
    println!("Enter user name:");
    let username = gets().unwrap();

    let otpauth_uri = format!(
        "otpauth://totp/{}:@{}?secret={}&issuer={}",
        service, username, key, service
    );

    display_qr_code(&otpauth_uri).expect("Couldn't display QR code");
    println!("Next couple of tokens: {:?}", present_codes(&key));
}

fn display_qr_code(otpauth_uri: &str) -> Result<(), io::Error> {
    let code = QrCode::new(otpauth_uri).unwrap();

    let string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("{}", string);
    Ok(())
}

fn present_codes(key: &str) -> Vec<String> {
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
    let value = ((((result[offset]) as i32 & 0x7f) << 24)
        | (((result[offset + 1]) as i32 & 0xff) << 16)
        | (((result[offset + 2]) as i32 & 0xff) << 8)
        | ((result[offset + 3]) as i32 & 0xff)) as i64;

    let length = 6;
    let pow10: i64 = 10;
    let modulo = value % pow10.pow(length);
    Ok(format!("{:0length$}", modulo, length = 6))
}

fn gets() -> io::Result<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => Ok(input.trim_end_matches('\n').to_string()),
        Err(error) => Err(error),
    }
}
