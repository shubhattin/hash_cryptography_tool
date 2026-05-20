//! Digest hashing (SHA-2, SHA-3), salt generation, and salted password digests.

use rand::RngCore;
use sha2::{Digest, Sha256, Sha512};
use sha3::{Sha3_256, Sha3_512};
use thiserror::Error;

const SALT_BYTES: usize = 16;

/// Supported digest algorithms for plain and salted password hashing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigestAlgorithm {
    Sha256,
    Sha512,
    Sha3_256,
    Sha3_512,
}

impl DigestAlgorithm {
    pub fn hex_length(self) -> usize {
        match self {
            Self::Sha256 | Self::Sha3_256 => 64,
            Self::Sha512 | Self::Sha3_512 => 128,
        }
    }
}

#[derive(Debug, Error)]
pub enum HashError {
    #[error("hash string is too short for {algorithm:?} (need at least {min} hex chars)")]
    InsufficientLength {
        algorithm: DigestAlgorithm,
        min: usize,
    },
}

/// Hex-encode bytes (lowercase).
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/// Decode hex string into bytes.
pub fn hex_to_bytes(hex_str: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex_str)
}

/// Random 16-byte salt as 32-character hex (matches the Svelte `gen_salt`).
pub fn generate_salt_hex() -> String {
    let mut salt = [0u8; SALT_BYTES];
    rand::rng().fill_bytes(&mut salt);
    bytes_to_hex(&salt)
}

/// Hash UTF-8 text with the chosen algorithm; returns lowercase hex.
pub fn hash_text(text: &str, algorithm: DigestAlgorithm) -> String {
    let bytes = text.as_bytes();
    match algorithm {
        DigestAlgorithm::Sha256 => {
            let digest = Sha256::digest(bytes);
            bytes_to_hex(&digest)
        }
        DigestAlgorithm::Sha512 => {
            let digest = Sha512::digest(bytes);
            bytes_to_hex(&digest)
        }
        DigestAlgorithm::Sha3_256 => {
            let digest = Sha3_256::digest(bytes);
            bytes_to_hex(&digest)
        }
        DigestAlgorithm::Sha3_512 => {
            let digest = Sha3_512::digest(bytes);
            bytes_to_hex(&digest)
        }
    }
}

/// Salted password digest: `hex(hash(password || salt_hex)) || salt_hex`.
pub fn hash_password_with_salt(
    password: &str,
    algorithm: DigestAlgorithm,
    salt_hex: &str,
) -> String {
    let digest = hash_text(&format!("{password}{salt_hex}"), algorithm);
    format!("{digest}{salt_hex}")
}

/// Build a new salted password hash (random salt appended, same format as Svelte).
pub fn hash_password_with_random_salt(password: &str, algorithm: DigestAlgorithm) -> String {
    let salt_hex = generate_salt_hex();
    hash_password_with_salt(password, algorithm, &salt_hex)
}

/// Verify a salted password hash produced by [`hash_password_with_salt`].
pub fn verify_password_with_salt(
    password: &str,
    stored: &str,
    algorithm: DigestAlgorithm,
) -> Result<bool, HashError> {
    let hex_len = algorithm.hex_length();
    if stored.len() < hex_len + 1 {
        return Err(HashError::InsufficientLength {
            algorithm,
            min: hex_len + 1,
        });
    }

    let (hash_part, salt_part) = stored.split_at(hex_len);
    let expected = hash_text(&format!("{password}{salt_part}"), algorithm);
    Ok(constant_time_eq_str(hash_part, &expected))
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
    fn sha256_known_vector() {
        let out = hash_text("abc", DigestAlgorithm::Sha256);
        assert_eq!(
            out,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn sha512_nonempty() {
        let out = hash_text("hello", DigestAlgorithm::Sha512);
        assert_eq!(out.len(), 128);
    }

    #[test]
    fn sha3_256_length() {
        let out = hash_text("hello", DigestAlgorithm::Sha3_256);
        assert_eq!(out.len(), 64);
    }

    #[test]
    fn salt_is_32_hex_chars() {
        let salt = generate_salt_hex();
        assert_eq!(salt.len(), 32);
        assert!(salt.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn salted_password_roundtrip() {
        let stored = hash_password_with_random_salt("secret", DigestAlgorithm::Sha256);
        assert!(stored.len() > 64);
        assert!(verify_password_with_salt("secret", &stored, DigestAlgorithm::Sha256).unwrap());
        assert!(!verify_password_with_salt("wrong", &stored, DigestAlgorithm::Sha256).unwrap());
    }

    #[test]
    fn salted_password_rejects_short_hash() {
        let err = verify_password_with_salt("x", "ab", DigestAlgorithm::Sha256).unwrap_err();
        assert!(matches!(err, HashError::InsufficientLength { .. }));
    }
}
