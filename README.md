# KG Password Generator written in Rust
Follows the original dart and flutter implementation ([Here](https://github.com/kghandour/kgpassgen)). 

This rust implementation aims to provide a CLI password generator and a cargo implementation to be used in other packages when needed.

For the CLI implementation, by default, the password input is masked from the terminal and the generated input gets automatically copied to clipboard without getting displayed to the terminal. This is done as a security feature to prevent possible logging to history. However, you will get prompted to show the generated password in the terminal. 

## Security features:
1. Generates a reproducible password that is unique to each different service
2. Generated passwords follows strict validations listed below
3. Masks the input password by default
4. Copies the generated password to clipboard by default
5. Does not show the generated password on the CLI by default

### KGPG Validations
1. Generated password must be at least 8 characters
2. Must contain a lowercase character, uppercase character, numbers, and special characters.

### SGP Validations
1. Generated password must be at least 8 characters
2. Must contain a lowercase character, uppercase character, and numbers.

## Linting
We use Clippy ([Here](https://github.com/rust-lang/rust-clippy)) for linting. Follow the installation process mentioned in the ([README.md](https://github.com/rust-lang/rust-clippy/blob/master/README.md)).

Run `cargo clippy` to check for linting issues.

## FAQ
1. If you are on Wayland and copying to clipboard does not work
This app uses `arboard` to manage the clipboard ([Here](https://github.com/1Password/arboard)). If there is a problem, it is recommended to use Xwayland instead. If the problem persists or you are facing it in other operating systems, please create an issue. 

An alternative solution is when prompted, you can show the generated password to the terminal.
