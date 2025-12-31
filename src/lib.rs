
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
        SHA512,
    }

    #[derive(Debug, PartialEq)]
    pub enum GeneratorType {
        KGPG,
        SGP,
    }

    #[derive(Debug)]
    pub struct Config {
        pub generator_type: GeneratorType,
        pub strip_subdomain: bool,
        pub hash_algorithm: HashAlgorithm,
        pub length: u8,
        pub hops: u8,
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                generator_type: GeneratorType::KGPG,
                strip_subdomain: true,
                hash_algorithm: HashAlgorithm::MD5,
                length: 15,
                hops: 15,
            }
        }
    }

    impl Config {
        pub const KGPG: Config = Config {
            generator_type: GeneratorType::KGPG,
            strip_subdomain: true,
            hash_algorithm: HashAlgorithm::MD5,
            length: 15,
            hops: 15,
        };

        pub const SGP: Config = Config {
            generator_type: GeneratorType::SGP,
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

    pub fn hash_md5(input: &str) -> String {
        format!("{:x}", md5::compute(input.as_bytes()))
    }

    pub fn hash_sha512(input: &str) -> String {
        use sha2::{Sha512, Digest};

        let mut hasher = Sha512::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn validate_password(password: &str, config: &Config) -> bool {
        let sliced_password = match password.get(0..config.length as usize) {
            Some(slice) => slice,
            None => return false,
        };

        let password_regex = fancy_regex::Regex::new(r"(?=.*^[a-z])(?=.*[A-Z])(?=.*[0-9])([a-zA-Z0-9#?!@$%^&*]){8,}$").unwrap();
        if !password_regex.is_match(sliced_password).unwrap() {
            return false;
        }

        if config.generator_type == GeneratorType::KGPG {
            let special_char_regex = fancy_regex::Regex::new(r"[^!#%@$&]").unwrap();
            if special_char_regex.is_match(sliced_password).unwrap() {
                return false;
            }
        }

        true
    }

    pub fn apply_kgpg (password: &str) -> String {
        let mut kgpg_password = String::new();
        for c in password.chars() {
            match c {
                '+' => kgpg_password.push('!'),
                '/' => kgpg_password.push('#'),
                '=' => kgpg_password.push('%'),
                '0' => kgpg_password.push('@'),
                '8' => kgpg_password.push('$'),
                '9' => kgpg_password.push('&'),
                _ => kgpg_password.push(c),
            }
        }
        kgpg_password
    }

    pub fn apply_sgp (password: &str) -> String {
        let mut sgp_password = String::new();
        for c in password.chars() {
            match c {
                '+' => sgp_password.push('9'),
                '/' => sgp_password.push('8'),
                '=' => sgp_password.push('A'),
                _ => sgp_password.push(c),
            }
        }
        sgp_password
    }

    pub fn apply_password_hops (password: &str, config: &Config) -> String {
        let mut hopped_password = password.to_string();
        let mut iteration = 0;
        while iteration < config.hops {
            println!("Hop number: {}", iteration);
            hopped_password = match config.hash_algorithm {
                HashAlgorithm::MD5 => hash_md5(&hopped_password),
                HashAlgorithm::SHA512 => hash_sha512(&hopped_password),
            };

            hopped_password = match config.generator_type {
                GeneratorType::KGPG => apply_kgpg(&hopped_password),
                GeneratorType::SGP => apply_sgp(&hopped_password),
            };

            println!("Hopped password: {}", hopped_password);

            if iteration == config.hops - 1 && !validate_password(&hopped_password, config) {
                println!("Reached loop {} and password validation is {}", iteration, validate_password(&hopped_password, config));
                iteration -= 1;
            }
            iteration += 1;
        }


        let sliced_password = match hopped_password.get(0..config.length as usize) {
            Some(slice) => slice,
            None => return hopped_password,
        };

        sliced_password.to_string()
    }

    pub fn generate_password(url: &str, master_password: &str, config: &Config) -> String {
        // Placeholder for password generation logic
        let host =  crate::url_helper::get_host(url, &config.strip_subdomain);

        let concat =format!("{}:{}", host.trim(), master_password.trim());
        apply_password_hops(&concat, config)
    }
}
