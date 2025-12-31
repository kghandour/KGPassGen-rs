#[derive(Debug, PartialEq)]
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
        hash_algorithm: HashAlgorithm::SHA512,
        length: 15,
        hops: 15,
    };

    pub const SGP: Config = Config {
        generator_type: GeneratorType::SGP,
        strip_subdomain: true,
        hash_algorithm: HashAlgorithm::SHA512,
        length: 10,
        hops: 10,
    };

    pub fn with_strip_subdomain(mut self, strip: bool) -> Self { self.strip_subdomain = strip ; self }
    pub fn with_hash_algorithm(mut self, algorithm: HashAlgorithm) -> Self { self.hash_algorithm = algorithm ; self }
    pub fn with_length(mut self, length: u8) -> Self { self.length = length ; self }
    pub fn with_hops(mut self, hops: u8) -> Self { self.hops = hops ; self }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.generator_type, GeneratorType::KGPG);
        assert_eq!(config.strip_subdomain, true);
        assert_eq!(config.hash_algorithm, HashAlgorithm::MD5);
        assert_eq!(config.length, 15);
        assert_eq!(config.hops, 15);
    }

    #[test]
    fn test_kgpg_config() {
        let config = Config::KGPG;
        assert_eq!(config.generator_type, GeneratorType::KGPG);
        assert_eq!(config.strip_subdomain, true);
        assert_eq!(config.hash_algorithm, HashAlgorithm::SHA512);
        assert_eq!(config.length, 15);
        assert_eq!(config.hops, 15);
    }

    #[test]
    fn test_sgp_config() {
        let config = Config::SGP;
        assert_eq!(config.generator_type, GeneratorType::SGP);
        assert_eq!(config.strip_subdomain, true);
        assert_eq!(config.hash_algorithm, HashAlgorithm::SHA512);
        assert_eq!(config.length, 10);
        assert_eq!(config.hops, 10);
    }

    #[test]
    fn test_config_builder_methods() {
        let config = Config::default()
            .with_strip_subdomain(false)
            .with_hash_algorithm(HashAlgorithm::SHA512)
            .with_length(20)
            .with_hops(5);

        assert_eq!(config.strip_subdomain, false);
        assert_eq!(config.hash_algorithm, HashAlgorithm::SHA512);
        assert_eq!(config.length, 20);
        assert_eq!(config.hops, 5);
    }
}