# QRForge

A Rust CLI to more safely generate a QR code from a 32-character TOTP secret key. Can also read QR codes from image files and present the TOTP secret key.

## The problem this tool tries to solve

**QR code --> TOTP secret key**: You're enabling two-factor authentication on an online account. The online service provides you with a QR code for you to take a photo of with your phone's authentication app (like Google Authenticator). That's all fine and good, but what if you want to save this QR code (or really, the secret key it contains) somewhere else, or share it with someone you trust? Taking a screenshot isn't ideal, both for convenience and security.

QRForge accepts an image file of the QR code and displays, or "reads," the discovered 32-character string that is the TOTP secret key, which you can write down on paper or paste into a password manager. To do this, you'd run `qrforge read <qr_code_image_file_path.png>`.

**TOTP secret key --> QR code**: You've got a 32-character TOTP secret and, for convenience, you want to generate, or "draw", a QR code so you can get it into your phone's authentication app. You can do this with QRForge by running `qrforge draw`. You'll then be prompted to enter the secret and other information about the account.

After you get through some prompts, a QR code will be displayed in your terminal. You'll also be given the choice to save the QR code to an image file.

### But is it actually secure?

Honestly, I'm not sure. But since QRForge uses [rpassword](https://github.com/conradkdotcom/rpassword) to take in the secret key, I figure it's better than using a generic tool for creating QR codes, like qrencode, which may store your secret key in your shell's history and potentially elsewhere.

### Other solutions

Know that [KeePassXC version 2.4.0 and above](https://keepassxc.org/) can generate TOTP QR codes (see [FAQ](https://keepassxc.org/docs/#faq-security-totp) and [relevant pull request](https://github.com/keepassxreboot/keepassxc/issues/1167)) and more. **If you can, I'd recommend using the latest version of KeePassXC rather than this tool for managing your TOTP keys and QR codes.** KeePassXC [has now been audited](https://keepassxc.org/blog/2023-04-15-audit-report/), and it generally has a lot more eyes on its code.

## Installation/Setup

1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. `cargo install --git https://github.com/sts10/qr-forge --branch main`

Alternatively: Clone repo, `cd` into repo directory, and run `cargo install --path=.`

## Usage

```text
USAGE:
    qrforge <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    draw    Draw a QR code from given TOTP text secret, service, and username
    help    Print this message or the help of the given subcommand(s)
    read    Read a QR code image file and prints OTPauth URI
```

The `draw` subcommand has its own options, including `output`:

```text
USAGE:
    qrforge draw [OPTIONS]

OPTIONS:
    -h, --help               Print help information
    -o, --output <OUTPUT>    Print created QR code to a file. I'd suggest using a .png file
                             extension
```

## Examples

- `qrforge read path/to/qr_code.png` _reads_ a TOTP secret from an image of a QR code.
- `qrforge draw` prompts the user to _draw_ or create a QR code from a TOTP secret and other information.
- `qrforge draw -o /path/to/new-created-qr-code.png` prompts user to create QR code and saves the resulting QR code to specified file.

![Demo of qrforge drawing a QR code from a TOTP secret and other account information, and displaying the resulting QR code](demo/demo.png)

## Limitations

This program's setting reading and creating TOTPs is hard-coded to some sensible default (30 seconds, one type of secret key, etc.). I also need to test it on image file types other than PNG.

## Troubleshooting

If generated codes aren't accepted by the online service, check to make sure your computer's time is accurate.

## Notes / reference

Here are [the official specifications of the otpauth URI format from Google](https://github.com/google/google-authenticator/wiki/Key-Uri-Format), if helpful.

Before I wrote this code, I wrote [a blog post](https://sts10.github.io/2018/11/26/totp-uris-qr-codes-2-factor.html) that might help you understand the problems I'm interested in here.

If you're just trying to create a QR code that points to a website (not a TOTP key), try [Dead Simple QR Code Generator](https://httpbin.dmuth.org/qrcode/) by dmuth.

## To do

- [x] Add ability to generate a few 6-digit codes, allowing users to confirm everything went right. See [this function](https://github.com/Skarlso/totp/blob/master/src/generator.rs#L9) for clues on how to do this.
- [x] Make this a real CLI using structopt or Clap
- [ ] Big refactor of the reading image code
- [ ] Provide ability to handle non-standard TOTP codes
