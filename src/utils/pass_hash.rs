//! Password hashing: bcrypt, Argon2, and scrypt (normal format only).

use argon2::{
    password_hash::{
        rand_core::OsRng, Error as PasswordHashError, PasswordHash, PasswordHasher,
        PasswordVerifier, SaltString,
    },
    Algorithm, Argon2, Params, Version,
};
use bcrypt::{hash_with_salt, verify, BcryptError, BcryptResult};
use rand::RngCore;
use scrypt::{scrypt, Params as ScryptParams};
use thiserror::Error;

use crate::utils::hash::{bytes_to_hex, hex_to_bytes};

/// Argon2 variant (matches Svelte pass_hash components).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgonVariant {
    Id,
    D,
    I,
}

/// Parameters for scrypt (maps to hash-wasm: costFactor, blockSize, parallelism, hashLength).
#[derive(Debug, Clone, Copy)]
pub struct ScryptConfig {
    /// CPU/memory cost N (power of two in the UI: 4, 8, 16, …).
    pub cost_factor: u32,
    pub block_size: u32,
    pub parallelism: u32,
    pub hash_length: usize,
}

impl Default for ScryptConfig {
    fn default() -> Self {
        Self {
            cost_factor: 8,
            block_size: 8,
            parallelism: 1,
            hash_length: 64,
        }
    }
}

#[derive(Debug, Error)]
pub enum PassHashError {
    #[error("bcrypt error: {0}")]
    Bcrypt(#[from] BcryptError),
    #[error("argon2 error: {0}")]
    Argon2(#[from] PasswordHashError),
    #[error("scrypt error: {0}")]
    Scrypt(String),
    #[error("invalid hash format: {0}")]
    InvalidFormat(&'static str),
    #[error("hex decode error: {0}")]
    Hex(#[from] hex::FromHexError),
    #[error("bcrypt hash must be 60 characters")]
    BcryptLength,
    #[error("argon2 hash is too short")]
    ArgonLength,
}

/// Hash a password with bcrypt; returns standard `$2b$…` encoded string.
pub fn bcrypt_hash(password: &str, cost: u32) -> BcryptResult<String> {
    let mut salt_bytes = [0u8; 16];
    rand::rng().fill_bytes(&mut salt_bytes);
    let parts = hash_with_salt(password, cost, salt_bytes)?;
    Ok(parts.to_string())
}

/// Verify password against a bcrypt hash string.
pub fn bcrypt_verify(password: &str, hash: &str) -> Result<bool, PassHashError> {
    if hash.len() != 60 {
        return Err(PassHashError::BcryptLength);
    }
    Ok(verify(password, hash)?)
}

/// Hash with Argon2 (encoded PHC string, like hash-wasm `outputType: 'encoded'`).
pub fn argon2_hash(
    password: &str,
    variant: ArgonVariant,
    parallelism: u32,
    iterations: u32,
    memory_size_kib: u32,
    hash_length: u32,
) -> Result<String, PassHashError> {
    let salt = SaltString::generate(&mut OsRng);
    let algorithm = match variant {
        ArgonVariant::Id => Algorithm::Argon2id,
        ArgonVariant::D => Algorithm::Argon2d,
        ArgonVariant::I => Algorithm::Argon2i,
    };

    let params = Params::new(
        memory_size_kib,
        iterations,
        parallelism,
        Some(hash_length as usize),
    )
    .map_err(|e| PassHashError::Argon2(e.into()))?;

    let argon2 = Argon2::new(algorithm, Version::V0x13, params);
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(PassHashError::Argon2)?;
    Ok(hash.to_string())
}

/// Verify password against an Argon2 PHC-encoded hash.
pub fn argon2_verify(password: &str, hash: &str) -> Result<bool, PassHashError> {
    if hash.len() < 50 {
        return Err(PassHashError::ArgonLength);
    }
    let parsed = PasswordHash::new(hash).map_err(|_| PassHashError::InvalidFormat("argon2"))?;
    // Algorithm is inferred from the hash string.
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

/// Normal scrypt hash: `{salt_hex}:{hash_hex}` (not Better Auth format).
pub fn scrypt_hash(password: &str, config: ScryptConfig) -> Result<String, PassHashError> {
    let mut salt = [0u8; 16];
    rand::rng().fill_bytes(&mut salt);
    let hash_hex = scrypt_derive_hex(password, &salt, config)?;
    Ok(format!("{}:{}", bytes_to_hex(&salt), hash_hex))
}

/// Verify normal scrypt `salt_hex:hash_hex` with the same parameters used at hash time.
pub fn scrypt_verify(
    password: &str,
    stored: &str,
    config: ScryptConfig,
) -> Result<bool, PassHashError> {
    let (salt_hex, expected_hex) = stored
        .split_once(':')
        .ok_or(PassHashError::InvalidFormat("scrypt salt:hash"))?;
    if salt_hex.is_empty() || expected_hex.is_empty() {
        return Err(PassHashError::InvalidFormat("scrypt empty part"));
    }

    let salt = hex_to_bytes(salt_hex)?;
    let computed = scrypt_derive_hex(password, &salt, config)?;
    Ok(constant_time_eq_str(&computed, expected_hex))
}

fn scrypt_derive_hex(
    password: &str,
    salt: &[u8],
    config: ScryptConfig,
) -> Result<String, PassHashError> {
    let log_n = cost_factor_to_log_n(config.cost_factor);
    let params = ScryptParams::new(
        log_n,
        config.block_size,
        config.parallelism,
        config.hash_length,
    )
    .map_err(|e| PassHashError::Scrypt(e.to_string()))?;

    let mut output = vec![0u8; config.hash_length];
    scrypt(password.as_bytes(), salt, &params, &mut output)
        .map_err(|e| PassHashError::Scrypt(e.to_string()))?;
    Ok(bytes_to_hex(&output))
}

/// Convert UI cost factor (N as power of two) to `log_n` for the Rust `scrypt` crate (`N = 2^log_n`).
fn cost_factor_to_log_n(cost_factor: u32) -> u8 {
    if cost_factor <= 1 {
        return 1;
    }
    (32 - cost_factor.leading_zeros() - 1) as u8
}

fn constant_time_eq_str(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    bool::from(subtle::ConstantTimeEq::ct_eq(a.as_bytes(), b.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bcrypt_roundtrip() {
        let hash = bcrypt_hash("password", 4).unwrap();
        assert!(hash.starts_with("$2"));
        assert_eq!(hash.len(), 60);
        assert!(bcrypt_verify("password", &hash).unwrap());
        assert!(!bcrypt_verify("wrong", &hash).unwrap());
    }

    #[test]
    fn bcrypt_rejects_short_hash() {
        assert!(matches!(
            bcrypt_verify("a", "short"),
            Err(PassHashError::BcryptLength)
        ));
    }

    #[test]
    fn argon2id_roundtrip() {
        let hash = argon2_hash("secret", ArgonVariant::Id, 2, 2, 256, 32).unwrap();
        assert!(hash.starts_with("$argon2"));
        assert!(argon2_verify("secret", &hash).unwrap());
        assert!(!argon2_verify("nope", &hash).unwrap());
    }

    #[test]
    fn scrypt_roundtrip_default_params() {
        let config = ScryptConfig::default();
        let stored = scrypt_hash("test-password", config).unwrap();
        assert!(stored.contains(':'));
        assert!(scrypt_verify("test-password", &stored, config).unwrap());
        assert!(!scrypt_verify("other", &stored, config).unwrap());
    }

    #[test]
    fn scrypt_rejects_bad_format() {
        let config = ScryptConfig::default();
        assert!(scrypt_verify("a", "nocolon", config).is_err());
    }

    #[test]
    fn cost_factor_to_log_n_values() {
        assert_eq!(cost_factor_to_log_n(8), 3);
        assert_eq!(cost_factor_to_log_n(1024), 10u8);
    }
}
