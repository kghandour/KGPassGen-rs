use std::io;
use arboard::Clipboard;

fn main() {
    let mut clipboard = Clipboard::new().unwrap();

    println!("===== KG Password Generator =====");
    println!("Enter the service url you want to generate a password for");
    println!("Service URL:");
    
    let mut url = String::new();

    io::stdin().read_line(&mut url)
        .expect("Failed to read input");

    let master_password = rpassword::prompt_password("Enter master password:\n").unwrap();

    let generated_password = generate_password(&url, &master_password);
    clipboard.set_text(&generated_password).unwrap();
    println!("Generated password copied to clipboard!");

    println!("Do you want to see the generated password? (y/n) [Default: n]:");
    let mut show_password = String::new();
    io::stdin().read_line(&mut show_password)
        .expect("Failed to read input");
    if show_password.trim().to_lowercase() == "y" || show_password.trim().to_lowercase() == "yes" {
        println!("Generated password: {}", generated_password.trim());
    } else {
        println!("Password not displayed.")
    }

}

fn generate_password(url: &str, master_password: &str) -> String {
    // Placeholder for password generation logic
    return format!("{}:{}", url.trim(), master_password.trim())
}
