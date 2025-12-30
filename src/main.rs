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

    let mut strip_subdomain_str = String::new();
    println!("Do you want to strip subdomains? (y/n) [Default: n]:");
    io::stdin().read_line(&mut strip_subdomain_str)
        .expect("Failed to read input");

    let strip_subdomain = matches!(strip_subdomain_str.trim().to_lowercase().as_str(), "y" | "yes");

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

    let generated_password = kg_passgen::generate_password(&url, &master_password, &strip_subdomain);
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
