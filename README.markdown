# QR Encoder

A Rust script to more safely generate a QR code from a 32-character TOTP secret key.

![Demo](demo.png)

## Notes

Know that [KeePassXC version 2.4.0 and beyond](https://keepassxc.org/) can generate TOTP QR codes ([see code](https://github.com/keepassxreboot/keepassxc/issues/1167)) and more. If you can, I'd recommend using KeePassXC and not this script for managing your TOTP keys and QR codes.

Before I wrote this code, I wrote [a blog post](https://sts10.github.io/2018/11/26/totp-uris-qr-codes-2-factor.html) that might help you understand the problems I'm interested in here. 

## Testing?

Not sure how. If you have ideas, create a PR or issue! 

