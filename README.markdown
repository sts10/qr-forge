# QR Encoder

A Rust script to more safely generate a QR code from a 32-character TOTP secret key.

![Demo](demo.png)

## Notes

Know that [KeePassXC version 2.4.0 and beyond](https://keepassxc.org/) can generate TOTP QR codes ([see code](https://github.com/keepassxreboot/keepassxc/issues/1167)) and more. If you can, I'd recommend using KeePassXC and not this script for managing your TOTP keys and QR codes.

Before I wrote this code, I wrote [a blog post](https://sts10.github.io/2018/11/26/totp-uris-qr-codes-2-factor.html) that might help you understand the problems I'm interested in here. 

## To Do 

Add ability to generate a few 6-digit codes, allowing users to confirm everything went right. See [this function](https://github.com/Skarlso/totp/blob/master/src/generator.rs#L9) for clues on how to do this.

### Testing?

Not sure how to go about testing the functionality of this script as is. If you have ideas, create a PR or issue! 
