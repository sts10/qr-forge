extern crate qrcode;
extern crate rpassword;
use qrcode::QrCode;
use std::io;

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

fn gets() -> io::Result<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => Ok(input.trim_end_matches('\n').to_string()),
        Err(error) => Err(error),
    }
}
