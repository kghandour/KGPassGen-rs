
pub mod url_helper {
    pub fn get_host(url: &str, strip_subdomain: &bool) -> String {
        // Placeholder for URL parsing logic
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
}

// TODO: Add Config enum with default values
// TODO: Add pattern matching to call different algorithms
pub mod generator {
    #[derive(Debug)]
    pub enum HashAlgorithm {
        MD5,
        SHA256,
        SHA512,
    }

    #[derive(Debug)]
    pub struct Config {
        pub strip_subdomain: bool,
        pub hash_algorithm: HashAlgorithm,
        pub length: u8,
        pub hops: u8,
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                strip_subdomain: true,
                hash_algorithm: HashAlgorithm::MD5,
                length: 15,
                hops: 15,
            }
        }
    }

    impl Config {
        pub const KGPG: Config = Config {
            strip_subdomain: true,
            hash_algorithm: HashAlgorithm::MD5,
            length: 15,
            hops: 15,
        };

        pub const SGP: Config = Config {
            strip_subdomain: true,
            hash_algorithm: HashAlgorithm::MD5,
            length: 10,
            hops: 10,
        };

        pub fn with_strip_subdomain(mut self, strip: bool) -> Self { self.strip_subdomain = strip ; self }
        pub fn with_hash_algorithm(mut self, algorithm: HashAlgorithm) -> Self { self.hash_algorithm = algorithm ; self }
        pub fn with_length(mut self, length: u8) -> Self { self.length = length ; self }
        pub fn with_hops(mut self, hops: u8) -> Self { self.hops = hops ; self }
    }

    pub fn generate_password(url: &str, master_password: &str, config: &Config) -> String {
        // Placeholder for password generation logic
        let host =  crate::url_helper::get_host(url, &config.strip_subdomain);

        format!("{}:{}", host.trim(), master_password.trim())
    }
}
