use qrforge::*;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
        let codes = read_codes_from_file(&args[1]);
        if codes.len() > 0 {
            println!("Discovered {} code(s):", codes.len());
            for code in codes {
                println!("{}", code);
            }
        } else {
            eprintln!("error!");
        }
        return;
    }
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
