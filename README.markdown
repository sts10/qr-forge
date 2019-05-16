# QR Forge

A Rust script to more safely generate a QR code from a 32-character TOTP secret key. Can also read QR codes from image files. This is an updated version of my [QR Encoder](https://github.com/sts10/qrencoder).

## The problem this tries to solve

You've got a TOTP secret key (say for 2-factor authentication). Rather than have to manually enter all 32 characters into a mobile app like Google Authenticator, we'd like to generate a QR code of this secret key. 

The problem with using an existing CLI like `qrencode` is that your secret key will be stored in your bash_history and elsewhere. Plus, it generates an image file, which you'll have to delete securely.

## What is does to try to solve this problem

This Rust script attempts to solve that by using [rpassword](https://github.com/conradkdotcom/rpassword) to take in the secret key, then using [qrcode-rust](https://github.com/kennytm/qrcode-rust) to display the generated QR code right in the terminal, rather than create an image file. This displayed QR code is high-quality enough for my iPhone to pick up accurately, and there's no generated image file to worry about.

### But is it actually secure? 

Honestly, I'm not sure. But I figure it's better than `qrencode -s 10 -o generated_twitter_qr_code.png 'otpauth://totp/Twitter:@sts10?secret=hereismysecret&issuer=Twitter'`

### Other solutions

Know that [KeePassXC version 2.4.0 and above](https://keepassxc.org/) can generate TOTP QR codes (see [FAQ](https://keepassxc.org/docs/#faq-security-totp) and [relevant pull request](https://github.com/keepassxreboot/keepassxc/issues/1167)) and more. If you can, I'd recommend using KeePassXC and not this script for managing your TOTP keys and QR codes.

## Setup

1. Install Rust
2. Clone repo, `cd` into repo directory

## Usage

- To **encode** a secret and create a QR code, run `cargo run`
- To **decode** a secret from an existing QR code image, run `cargo run -- -d=<qr_code_image_file_path.png>`

![Demo](demo/demo.png)


## Notes

Before I wrote this code, I wrote [a blog post](https://sts10.github.io/2018/11/26/totp-uris-qr-codes-2-factor.html) that might help you understand the problems I'm interested in here. 

## To Do 

- [x] Add ability to generate a few 6-digit codes, allowing users to confirm everything went right. See [this function](https://github.com/Skarlso/totp/blob/master/src/generator.rs#L9) for clues on how to do this.
- [ ] Big refactor of the reading image code
- [ ] Make this a real CLI using structopt or Clap
