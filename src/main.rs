use std::io;
use arboard::Clipboard;
use inquire::{Confirm, CustomType, Password, Select, Text, required, validator::Validation};

fn main() {
    println!("KG Password Generator");
    let mut clipboard = match Clipboard::new() {
        Ok(clipboard) => clipboard,
        Err(e) => {
            println!("Warning: Clipboard unavailable; generated password will not be copied. ({})", e);
            return;
        }
    };

    let master_password = Password::new("Master Password:")
        .with_help_message("Your master password used to derive service passwords")
        .with_display_mode(inquire::PasswordDisplayMode::Masked)
        .with_validator(required!("A master password is required"))
        .without_confirmation()
        .prompt()
        .expect("Failed to read input");

    let url = Text::new("Service URL:")
        .with_placeholder("e.g., https://example.com")
        .with_help_message("The website or service you are trying to generate a password for")
        .prompt()
        .expect("Failed to read input");

    let kg_config = format!("KGPG {:?}", kg_passgen::config::Config::KGPG);
    let sgp_config = format!("SGP {:?}", kg_passgen::config::Config::SGP);
    let select_config = Select::new("Select Configuration", vec![&kg_config, &sgp_config, "Custom"])
        .with_help_message("Choose the password generation configuration")
        .prompt();

    let config = match select_config {
        Ok(choice) => {
            if choice == kg_config {
                kg_passgen::config::Config::KGPG
            } else if choice == sgp_config {
                kg_passgen::config::Config::SGP
            } else {
                let strip_domain = Confirm::new("Strip Subdomain?")
                    .with_help_message("Whether to remove subdomains from the URL host")
                    .with_default(true)
                    .prompt()
                    .expect("Failed to read input");

                let hash_algorithm_select = Select::new("Select Hash Algorithm", vec!["SHA512", "MD5"])
                    .with_help_message("Choose the hashing algorithm for password generation")
                    .prompt()
                    .expect("Failed to read input");

                let hash_algorithm = match hash_algorithm_select {
                    "MD5" => kg_passgen::config::HashAlgorithm::MD5,
                    "SHA512" => kg_passgen::config::HashAlgorithm::SHA512,
                    _ => kg_passgen::config::HashAlgorithm::SHA512,
                };

                let hash_algorithm_clone = hash_algorithm.clone();

                let number_input_validtion = move |input: &u8| {
                    let md5_condition = hash_algorithm_clone == kg_passgen::config::HashAlgorithm::MD5 && *input >= 8 && *input <= 24;
                    let sha512_condition = hash_algorithm_clone == kg_passgen::config::HashAlgorithm::SHA512 && *input >= 8 && *input <= 84;
                    if md5_condition || sha512_condition {
                        Ok(Validation::Valid)
                    } else {
                        Ok(Validation::Invalid("When using MD5, the generated password must be between 8 and 24 characters. When using SHA512, it must be between 8 and 84 characters.".into()))
                    }
                };

                let length: u8 = CustomType::<u8>::new("Password Length:")
                    .with_help_message("Desired length of the generated password")
                    .with_validator(number_input_validtion)
                    .with_error_message("Please enter a valid number")
                    .with_placeholder("e.g., 15")
                    .prompt()
                    .expect("Failed to read input");

                let hops: u8 =  CustomType::<u8>::new("Number of Hops:")
                    .with_help_message("Number of hashing iterations to apply")
                    .with_error_message("Please enter a valid number")
                    .with_placeholder("e.g., 15")
                    .prompt()
                    .expect("Failed to read input");

                kg_passgen::config::Config::default()
                    .with_hash_algorithm(hash_algorithm)
                    .with_length(length)
                    .with_hops(hops)
                    .with_strip_subdomain(strip_domain)
            }
        },
        Err(_) => {
            println!("Error selecting configuration, defaulting to KGPG.");
            kg_passgen::config::Config::KGPG
        }
    };

    println!("Current configuration is: {:?}", config);

    let generated_password = match kg_passgen::generator::generate_password(&url, &master_password, &config) {
        Ok(pw) => pw,
        Err(e) => {
            println!("Error generating password: {}.", e);
            return;
        }
    };

    match clipboard.set_text(generated_password.clone()) {
        Ok(_) => {
            println!("Generated password copied to clipboard!");
        },
        Err(e) => {
            println!("Failed to copy to clipboard: {}.", e);
        }
    }

    let show_password = Confirm::new("Show generated password?")
        .with_help_message("Choose whether to display the generated password in the console")
        .with_default(false)
        .prompt()
        .expect("Failed to read input");

    if show_password {
        println!("Generated password: \n{}", generated_password.trim());
    } else {
        println!("Password not displayed.")
    }

    println!("Press Enter to exit...");
    io::stdin().read_line(&mut String::new()).expect("Failed to detect input. Exiting");
}
