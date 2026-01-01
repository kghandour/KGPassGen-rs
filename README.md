# KG Password Generator written in Rust
[![Latest version](https://img.shields.io/crates/v/kg_passgen?color=mediumvioletred)](https://crates.io/crates/kg_passgen)
[![Documentation](https://docs.rs/kg_passgen/badge.svg)](https://docs.rs/kg_passgen)

![CLI](./.github/assets/kg_passgen.gif)

Follows the original dart and flutter implementation ([Here](https://github.com/kghandour/kgpassgen)). 

This rust implementation aims to provide a CLI password generator and a cargo implementation to be used in other packages when needed.

Check the library documentation ([Here](https://docs.rs/kg_passgen)).

For the CLI implementation, by default, the password input is masked from the terminal and the generated input gets automatically copied to clipboard without getting displayed to the terminal. This is done as a security feature to prevent possible logging to history. However, you will get prompted to show the generated password in the terminal. 

## Installation
`kg_passgen`is available on [crates.io](https://crates.io/crates/kg_passgen).
It is offered both as a binary CLI and as a library

### Binary Installation:
Simply run
```shell
cargo install kg_passgen
```

Then you should have access to the password generator. To start it type:
```shell
kg_passgen
```

and follow the prompts.


### Install as a Library
Simply run
```shell
cargo add kg_passgen
```

## Example
```rust
use kg_passgen::config::{Config, HashAlgorithm, GeneratorType};
use kg_passgen::generator::generate_password;
let config = Config::KGPG;

let example_password = generate_password("https://example.com", "my_master_password", &config).unwrap();
assert_eq!(example_password.len(), config.length as usize);
assert_eq!(example_password, "mXApUt1OgTb$xZh");

let different_password = generate_password("https://test.com", "my_master_password", &config).unwrap();
assert_eq!(different_password.len(), config.length as usize);
assert_eq!(different_password, "jtNRe$VWbnE#F6y");
```

For more information checkout the different modules available through the docs ([Here](https://docs.rs/kg_passgen)).


## Security features:
1. Generates a reproducible password that is unique to each different service
2. Passwords are hashed multiple times till they satisfy the validation
3. Generated passwords follows strict validations listed below
4. Masks the input password by default
5. Copies the generated password to clipboard by default
6. Does not show the generated password on the CLI by default

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

This app uses `arboard` to manage the clipboard ([Here](https://github.com/1Password/arboard)). If there is a problem, it is recommended to use Xwayland instead. If the problem persists or you are facing it in other operating systems, please create an issue. An alternative solution is when prompted, you can show the generated password to the terminal.

2. Installation fails with "linker `cc` not found"

You are missing a C Linker. You can use build-essentials if you are on Linux for example

```
sudo apt install build-essential
```


## License
Licensed under [MIT license](LICENSE.md).
