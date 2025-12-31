pub fn get_host(url: &str, strip_subdomain: &bool) -> String {
    match url::Url::parse(url) {
        Ok(parsed) => {
            let mut host = parsed.host_str().unwrap_or(url).to_string();

            if *strip_subdomain {
                host = match psl::domain_str(host.as_str()) {
                    Some(domain) => domain.to_string(),
                    None => host,
                };
            }

            host
        },
        Err(_) => url.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_host_with_strip_subdomain() {
        let url = "https://sub.example.co.uk/path";
        let host = get_host(url, &true);
        assert_eq!(host, "example.co.uk");
    }

    #[test]
    fn test_get_host_without_strip_subdomain() {
        let url = "https://sub.example.co.uk/path";
        let host = get_host(url, &false);
        assert_eq!(host, "sub.example.co.uk");
    }

    #[test]
    fn test_get_host_invalid_url() {
        let url = "not a valid url";
        let host = get_host(url, &true);
        assert_eq!(host, "not a valid url");
    }
}