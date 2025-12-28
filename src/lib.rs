
pub mod url_helper {
    pub fn parse_url(url: &str, strip_subdomain: bool) -> Result<String, url::ParseError> {
        // Placeholder for URL parsing logic
        let parsed = url::Url::parse(url)?;
        let host = parsed.host_str().unwrap_or("").to_string();

        let main_domain = match host.find('.') {
            Some(pos) => &host[pos + 1..],
            None => &host,
        };

        if strip_subdomain {
            Ok(main_domain.to_string())
        } else {
            Ok(host)
        }
    }
}