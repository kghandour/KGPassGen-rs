use std::io;

fn main() {
    println!("===== KG Password Generator =====");
    println!("Enter the service url you want to generate a password for");
    println!("Service URL:");
    
    let mut url = String::new();

    io::stdin().read_line(&mut url)
        .expect("Failed to read input");

    let master_password = rpassword::prompt_password("Enter master password:\n").unwrap();

    println!("Generating an MD5 hash for {}:{}", url.trim(), master_password.trim());

}
