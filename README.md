No More Secrets
===

This was done as an exercise to practice writing Rust code. The original
no-more-secrets program can be found [here](https://github.com/bartobri/no-more-secrets).

Usage
===

Just pipe the text into the program
```sh
cat text | ./nms-rs
```
Note: it supports ANSI control codes for SGR but it is bugged. Any other ANSI
control code is not covered. (For example, `neofetch` uses ANSI control codes
that are not SGR to print the info right next to the logo.
