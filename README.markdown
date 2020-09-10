# QRForge

A Rust CLI to more safely generate a QR code from a 32-character TOTP secret key. Can also read QR codes from image files and present the TOTP secret key.

## The problem this tool tries to solve

**QR code --> TOTP secret key**: You're enabling two-factor authentication on an online account. A service provides you with QR code for you to take a photo of with your phone's authentication app (like Google Authenticator). That's all fine and good, but what if you want to save this QR code (or really, the secret key it contains) somewhere else, or share it with someone you trust?

QRForge accepts an image file of the QR code and displays or "reads" the discovered 32-character string that is the TOTP secret key, which you can write down on paper or paste into a password manager. To do this, you'd run `qrforge read <qr_code_image_file_path.png>` 

**TOTP secret key --> QR code**: You've got a 32-character TOTP secret and, for convenience, you want to generate, or "draw", a QR code so you can get it into your phone's authentication app. You can do this with QRForge by running `qrforge draw`. You'll then be prompted to enter the secret and other information about the account. 

After you get through some prompts, a QR code will be displayed in your terminal. You'll also be given the choice to save the QR code to an image file.

### But is it actually secure? 

Honestly, I'm not sure. But since QRForge uses [rpassword](https://github.com/conradkdotcom/rpassword) to take in the secret key, I figure it's better than using a generic tool for creating QR codes, like qrencode, which may store your secret key in your shell's history and potentially elsewhere.

### Other solutions

Know that [KeePassXC version 2.4.0 and above](https://keepassxc.org/) can generate TOTP QR codes (see [FAQ](https://keepassxc.org/docs/#faq-security-totp) and [relevant pull request](https://github.com/keepassxreboot/keepassxc/issues/1167)) and more. If you can, I'd recommend using KeePassXC rather than this tool for managing your TOTP keys and QR codes.

## Installation/Setup

1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. `cargo install --git https://github.com/sts10/qr-forge`

Alternatively: Clone repo, `cd` into repo directory, and run `cargo install --path=.`

## Usage

```text
USAGE:
qrforge <SUBCOMMAND>

FLAGS:
-h, --help       Prints help information
-V, --version    Prints version information

SUBCOMMANDS:                                                                                                                                                                                                                                                                                           
draw    Draw a QR code from text secret, service, and username                                                                                                                                                                                                                                     
help    Prints this message or the help of the given subcommand(s)                                                                                                                                                                                                                                 
read    Read a QR code image file to an OTPauth URI  
```

Basically...

- To **read** a secret from an existing QR code image, run `qrforge read <qr_code_image_file_path.png>`
- To **draw** a QR code from a secret, run `qrforge draw`. You'll then be prompted for specifics.

![Demo of qrforge encoding a TOTP secret and displaying results QR code](demo/demo.png)

## Limitations

This program's setting reading and creating TOTPs is hard-coded to some sensible default (30 seconds, etc.).  

## Notes / reference

Here are [the official specifications of the otpauth URI format from Google](https://github.com/google/google-authenticator/wiki/Key-Uri-Format), if helpful.

Before I wrote this code, I wrote [a blog post](https://sts10.github.io/2018/11/26/totp-uris-qr-codes-2-factor.html) that might help you understand the problems I'm interested in here. 

## To do 

- [x] Add ability to generate a few 6-digit codes, allowing users to confirm everything went right. See [this function](https://github.com/Skarlso/totp/blob/master/src/generator.rs#L9) for clues on how to do this.
- [x] Make this a real CLI using structopt or Clap
- [ ] Big refactor of the reading image code
- [ ] Provide ability to handle non-standard TOTP codes.
