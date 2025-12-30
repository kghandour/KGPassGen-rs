use std::io;
use arboard::Clipboard;

// TODO: Add argparsing for CLI implementation
fn main() {
    let mut clipboard = match Clipboard::new() {
        Ok(clipboard) => clipboard,
        Err(e) => {
            println!("Warning: Clipboard unavailable; generated password will not be copied. ({})", e);
            return;
        }
    };

    println!("===== KG Password Generator =====");
    println!("Enter the service url you want to generate a password for");
    println!("Service URL:");
    
    let mut url = String::new();

    io::stdin().read_line(&mut url)
        .expect("Failed to read input");

    // let mut strip_subdomain_str = String::new();
    // println!("Do you want to strip subdomains? (y/n) [Default: n]:");
    // io::stdin().read_line(&mut strip_subdomain_str)
    //     .expect("Failed to read input");

    // let strip_subdomain = matches!(strip_subdomain_str.trim().to_lowercase().as_str(), "y" | "yes");

    let mut master_password = String::new();
    while master_password.trim().is_empty() {
        master_password = match rpassword::prompt_password("Enter master password:\n") {
            Ok(pw) => pw,
            Err(_) => {
                println!("Failed to read master password. Please try again.");
                continue;
            }
        };
        if master_password.trim().is_empty() {
            println!("Master password cannot be empty. Please try again.");
        }
    }

    println!("Please pick a configuration for the password generator:");
    println!("1) KGPG (Default): {:?}", kg_passgen::generator::Config::KGPG);
    println!("2) SGP: {:?}", kg_passgen::generator::Config::SGP);
    println!("Enter choice (1 or 2) [Default: 1]:");
    let mut config_choice = String::new();
    io::stdin().read_line(&mut config_choice).expect("Failed to read input");

    let config = match config_choice.trim() {
        "2" => kg_passgen::generator::Config::SGP,
        _ => kg_passgen::generator::Config::KGPG,
    };

    println!("Current configuration is: {:?}", config);

    let generated_password = kg_passgen::generator::generate_password(&url, &master_password, &config);
    match clipboard.set_text(generated_password.clone()) {
        Ok(_) => {
            println!("Generated password copied to clipboard!");
        },
        Err(e) => {
            println!("Failed to copy to clipboard: {}.", e);
        }
    }

    println!("Do you want to see the generated password? (y/n) [Default: n]:");
    let mut show_password = String::new();
    io::stdin().read_line(&mut show_password)
        .expect("Failed to read input");
    if show_password.trim().to_lowercase() == "y" || show_password.trim().to_lowercase() == "yes" {
        println!("Generated password: {}", generated_password.trim());
    } else {
        println!("Password not displayed.")
    }

    println!("Press Enter to exit...");
    io::stdin().read_line(&mut String::new()).expect("Failed to detect input. Exiting");

}
