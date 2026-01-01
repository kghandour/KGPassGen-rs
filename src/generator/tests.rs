use super::*;
use base64::prelude::BASE64_STANDARD;
use sha2::{Sha512, Digest};
use crate::config::{Config, GeneratorType};

#[test]
fn test_apply_kgpg_replacements() {
    let input = "+/=089z";
    let out = apply_kgpg(input);
    assert_eq!(out, "!#%@$&z");
}

#[test]
fn test_apply_sgp_replacements() {
    let input = "+/=abc";
    let out = apply_sgp(input);
    // '+' => '9', '/' => '8', '=' => 'A'
    assert_eq!(out, "98Aabc");
}

#[test]
fn test_validate_password_basic_success_and_failure() {
    let mut config = Config::default()
        .with_length(8)
        .with_generator_type(GeneratorType::SGP);

    // valid: has lowercase, uppercase and digit, 8 chars
    assert!(validate_password("aA1aaaaa", &config));

    // too short -> fails due to slicing returning None
    config.length = 10;
    assert!(!validate_password("aA1", &config));
}

#[test]
fn test_validate_password_kgpg_requires_special_char() {
    let config = Config::default()
        .with_length(9)
        .with_generator_type(GeneratorType::KGPG);

    // has required special char '!'
    assert!(validate_password("aA1aaaaa!", &config));

    // same without special -> should fail
    assert!(!validate_password("aA1aaaaaa", &config));
}

#[test]
fn test_apply_password_hops_zero_hops_returns_input_unmodified() {
    let config = Config::default()
        .with_hops(0)
        .with_length(20); // larger than input so slicing will return None and original returned

    let input = "short";
    let out = apply_password_hops(input, &config).unwrap();
    assert_eq!(out, input);
}

#[test]
fn test_generate_password_uses_get_host_and_trims() {
    let config = Config::default()
        .with_hops(0) // avoid hashing so we can assert exact concatenation result
        .with_length(50)// long enough to not slice
        .with_strip_subdomain(true);

    let url = "https://sub.example.co.uk/path";
    let master = "  master  ";
    let pw = generate_password(url, master, &config).unwrap();
    assert_eq!(pw, "master:example.co.uk");
}

#[test]
fn test_generate_password_with_invalid_url_uses_raw_url() {
    let config = Config::default()
        .with_hops(0)
        .with_length(80)
        .with_strip_subdomain(true);

    let url = "notavalidurl";
    let master = "m";
    let pw = generate_password(url, master, &config).unwrap();
    assert_eq!(pw, "m:notavalidurl");
}

#[test]
fn test_hash_md5_decodes_to_16_bytes_and_matches_md5() {
    let input = "abc";
    let hashed = hash_md5(input);
    let decoded = BASE64_STANDARD.decode(&hashed).unwrap();
    assert_eq!(decoded.len(), 16);
    let expected = md5::compute(input.as_bytes());
    assert_eq!(decoded, expected.0.to_vec());
}

#[test]
fn test_hash_sha512_decodes_to_64_bytes_and_matches_sha512() {
    let input = "abc";
    let hashed = hash_sha512(input);
    let decoded = BASE64_STANDARD.decode(&hashed).unwrap();
    assert_eq!(decoded.len(), 64);
    let mut hasher = Sha512::new();
    hasher.update(input.as_bytes());
    let expected = hasher.finalize();
    assert_eq!(decoded, expected.as_slice());
}

#[test]
fn test_generate_password_produces_different_passwords_for_different_urls() {
    let config = Config::KGPG;

    let master = "my_master_password";

    let example_password = generate_password("https://example.com", master, &config).unwrap();
    let different_password = generate_password("https://test.com", master, &config).unwrap();

    assert_ne!(example_password, different_password);
    assert_eq!(example_password, "mXApUt1OgTb$xZh");
    assert_eq!(different_password, "jtNRe$VWbnE#F6y");
}
#[test]
fn test_raises_an_error_for_invalid_length_md5() {
    let config = Config::default()
        .with_hash_algorithm(HashAlgorithm::MD5)
        .with_length(30) // invalid length for MD5
        .with_hops(1);

    let result = generate_password("https://example.com", "master", &config);
    assert!(result.is_err());
    assert!(matches!(result, Err(InvalidLengthError)));
}

#[test]
fn test_raises_an_error_for_invalid_length_sha512() {
    let config = Config::default()
        .with_hash_algorithm(HashAlgorithm::SHA512)
        .with_length(100) // invalid length for SHA512
        .with_hops(1);

    let result = generate_password("https://example.com", "master", &config);
    assert!(result.is_err());
    assert!(matches!(result, Err(InvalidLengthError)));
}