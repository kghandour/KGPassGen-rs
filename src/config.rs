//! Configuration module for the password generator application.
//! Defines the Config struct and related enums for hash algorithms and generator types.
//! Also includes default configurations for KGPG and SGP.
//! When using the GeneratorType::KGPG, it adds an extra security layer 
//! by making sure that it includes symbols in the generated passwords.
//! 
//! # Examples
//! ```
//! use kg_passgen::config::{Config, HashAlgorithm, GeneratorType};
//! let custom_config = Config::default()
//!     .with_hash_algorithm(HashAlgorithm::SHA512)
//!     .with_length(20)
//!     .with_hops(5)
//!     .with_strip_subdomain(false);
//! 
//! assert_eq!(custom_config.hash_algorithm, HashAlgorithm::SHA512);
//! assert_eq!(custom_config.length, 20);
//! assert_eq!(custom_config.hops, 5);
//! assert_eq!(custom_config.strip_subdomain, false);
//! ```
//! It is also possible to directly create a config instance:
//! ```
//! use kg_passgen::config::{Config, HashAlgorithm, GeneratorType};
//! let sgp_config = Config {
//!    generator_type: GeneratorType::SGP,
//!   strip_subdomain: true,
//!   ..Default::default()
//! };
//! assert_eq!(sgp_config.generator_type, GeneratorType::SGP);
//! assert!(sgp_config.strip_subdomain);
//! assert_eq!(sgp_config.hash_algorithm, HashAlgorithm::MD5);
//! assert_eq!(sgp_config.length, 15);
//! assert_eq!(sgp_config.hops, 15);
//! ```
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
    pub fn with_generator_type(mut self, generator_type: GeneratorType) -> Self { self.generator_type = generator_type ; self }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.generator_type, GeneratorType::KGPG);
        assert!(config.strip_subdomain);
        assert_eq!(config.hash_algorithm, HashAlgorithm::MD5);
        assert_eq!(config.length, 15);
        assert_eq!(config.hops, 15);
    }

    #[test]
    fn test_kgpg_config() {
        let config = Config::KGPG;
        assert_eq!(config.generator_type, GeneratorType::KGPG);
        assert!(config.strip_subdomain);
        assert_eq!(config.hash_algorithm, HashAlgorithm::SHA512);
        assert_eq!(config.length, 15);
        assert_eq!(config.hops, 15);
    }

    #[test]
    fn test_sgp_config() {
        let config = Config::SGP;
        assert_eq!(config.generator_type, GeneratorType::SGP);
        assert!(config.strip_subdomain);
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
            .with_hops(5)
            .with_generator_type(GeneratorType::KGPG);

        assert_eq!(config.generator_type, GeneratorType::KGPG);
        assert!(!config.strip_subdomain);
        assert_eq!(config.hash_algorithm, HashAlgorithm::SHA512);
        assert_eq!(config.length, 20);
        assert_eq!(config.hops, 5);
    }
}