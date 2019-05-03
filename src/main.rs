use qrencoder::*;

fn main() {
    let key: String = get_key();
    let service = gets("Enter name of service").unwrap();
    let username = gets("Enter username").unwrap();

    let otpauth_uri = make_otpauth_uri(&key, service, username);

    display_qr_code(&otpauth_uri).expect("Couldn't display QR code");

    println!("Next couple of tokens: {:?}", present_codes(&key));
}
