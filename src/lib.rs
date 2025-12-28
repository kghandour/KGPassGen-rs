
pub mod url_helper {
    pub fn get_host(url: &str, strip_subdomain: bool) -> String {
        // Placeholder for URL parsing logic
        match url::Url::parse(url) {
            Ok(parsed) => {
                let mut host = parsed.host_str().unwrap_or(url).to_string();

        let main_domain = match host.find('.') {
            Some(pos) => &host[pos + 1..],
            None => &host,
        };

        if strip_subdomain {
            host = main_domain.to_string();
        } 

        return host;
            },
            Err(_) => return url.to_string(),
        }
        
    }
}

pub fn generate_password(url: &str, master_password: &str, strip_subdomain: &bool) -> String {
    // Placeholder for password generation logic
    let host =  crate::url_helper::get_host(url, *strip_subdomain);

    return format!("{}:{}", host.trim(), master_password.trim());
}
