# QR Forge

A Rust CLI to more safely generate a QR code from a 32-character TOTP secret key. Can also read QR codes from image files and present the TOTP secret key.

## The problem this tool tries to solve

**QR code --> TOTP secret key**: You're enabling two-factor authentication on an online account. A service provides you with QR code for you to take a photo of with your phone's authentication app (like Google Authenticator). That's all fine and good, but what if you want to save this QR code (or really, the secret key it contains) somewhere else?

QR Forge accepts an image file of the QR code and displays the discovered 32-character string that is the TOTP secret key, which you can write down on paper or paste into a password manager. To do this, you'd run `qrforge -d <qr_code_image_file_path.png>` (`d` for "decoding" the image).

**TOTP secret key --> QR code**: You've got a 32-character TOTP secret and, for convenience, you want to generate a QR code so you can get it into your phone's authentication app. You can do this with QR Forge by running `qrforge -e`. You'll then be prompted to enter the secret and other information about the service. 

After you get through some prompts, a QR code will be displayed in your terminal, additionally you'll be given the choice to save the QR code to an image file.

### But is it actually secure? 

Honestly, I'm not sure. But since QR Forge uses [rpassword](https://github.com/conradkdotcom/rpassword) to take in the secret key, I figure it's better than `qrencode -s 10 -o generated_twitter_qr_code.png 'otpauth://totp/Twitter:@sts10?secret=hereismysecret&issuer=Twitter'`, which will store your secret in your BASH history and potentially elsewhere.

### Other solutions

Know that [KeePassXC version 2.4.0 and above](https://keepassxc.org/) can generate TOTP QR codes (see [FAQ](https://keepassxc.org/docs/#faq-security-totp) and [relevant pull request](https://github.com/keepassxreboot/keepassxc/issues/1167)) and more. If you can, I'd recommend using KeePassXC and not this script for managing your TOTP keys and QR codes.

## Installation/Setup

1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. `cargo install --git https://github.com/sts10/qr-forge`

Alternatively: Clone repo, `cd` into repo directory, and run `cargo install --path=.`

## Usage

```text
USAGE:
    qrforge [FLAGS] [OPTIONS]

FLAGS:
    -e, --encode     Encode QR code from text secret, service and username
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --decode <qr_image_file>    Decode a QR code image file to an OTPauth URI

```

Basically...

- To **decode** a secret from an existing QR code image, run `qrforge -d=<qr_code_image_file_path.png>`
- To **encode** a secret and create a QR code, run `qrforge -e`. You'll then be prompted for information.

![Demo](demo/demo.png)


## Notes / reference

Here are [the official-ish specs of the otpauth URI from Google](https://github.com/google/google-authenticator/wiki/Key-Uri-Format).

Before I wrote this code, I wrote [a blog post](https://sts10.github.io/2018/11/26/totp-uris-qr-codes-2-factor.html) that might help you understand the problems I'm interested in here. 

## To do 

- [x] Add ability to generate a few 6-digit codes, allowing users to confirm everything went right. See [this function](https://github.com/Skarlso/totp/blob/master/src/generator.rs#L9) for clues on how to do this.
- [x] Make this a real CLI using structopt or Clap
- [ ] Big refactor of the reading image code
