//! Authenticated encryption using AES-256-GCM (not the legacy AES-CBC + WebCrypto path from the Svelte app).
//!
//! Passphrase → SHA-256 → 32-byte key. Ciphertext format: `{base64(nonce)}-{base64(ciphertext+tag)}`.

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use rand::RngCore;
use sha2::{Digest, Sha256};
use thiserror::Error;

const NONCE_LEN: usize = 12;

#[derive(Debug, Error)]
pub enum EncryptError {
    #[error("invalid ciphertext format")]
    InvalidFormat,
    #[error("base64 decode failed")]
    Base64Decode,
    #[error("decryption failed (wrong key or corrupted data)")]
    DecryptFailed,
    #[error("encryption failed")]
    EncryptFailed,
}

/// Derive a 32-byte AES-256 key from a passphrase (SHA-256 of UTF-8 passphrase).
pub fn derive_aes256_key(passphrase: &str) -> [u8; 32] {
    let digest = Sha256::digest(passphrase.as_bytes());
    digest.into()
}

/// Encrypt plaintext; returns `nonce_b64-ciphertext_b64`.
pub fn encrypt_text(plaintext: &str, passphrase: &str) -> Result<String, EncryptError> {
    let key = derive_aes256_key(passphrase);
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| EncryptError::EncryptFailed)?;

    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|_| EncryptError::EncryptFailed)?;

    Ok(format!(
        "{}-{}",
        B64.encode(nonce_bytes),
        B64.encode(ciphertext)
    ))
}

/// Decrypt a string produced by [`encrypt_text`].
pub fn decrypt_text(payload: &str, passphrase: &str) -> Result<String, EncryptError> {
    let (nonce_b64, ct_b64) = payload
        .split_once('-')
        .ok_or(EncryptError::InvalidFormat)?;

    let nonce_bytes = B64
        .decode(nonce_b64)
        .map_err(|_| EncryptError::Base64Decode)?;
    if nonce_bytes.len() != NONCE_LEN {
        return Err(EncryptError::InvalidFormat);
    }

    let ciphertext = B64
        .decode(ct_b64)
        .map_err(|_| EncryptError::Base64Decode)?;

    let key = derive_aes256_key(passphrase);
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| EncryptError::DecryptFailed)?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|_| EncryptError::DecryptFailed)?;

    String::from_utf8(plaintext).map_err(|_| EncryptError::DecryptFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_key_is_deterministic() {
        let a = derive_aes256_key("my-key");
        let b = derive_aes256_key("my-key");
        assert_eq!(a, b);
        assert_ne!(derive_aes256_key("other"), a);
    }

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let enc = encrypt_text("hello world", "secret").unwrap();
        assert!(enc.contains('-'));
        let dec = decrypt_text(&enc, "secret").unwrap();
        assert_eq!(dec, "hello world");
    }

    #[test]
    fn decrypt_wrong_key_fails() {
        let enc = encrypt_text("data", "key-a").unwrap();
        assert!(matches!(
            decrypt_text(&enc, "key-b"),
            Err(EncryptError::DecryptFailed)
        ));
    }

    #[test]
    fn decrypt_garbage_fails() {
        assert!(matches!(
            decrypt_text("notvalid", "k"),
            Err(EncryptError::InvalidFormat)
        ));
        assert!(matches!(
            decrypt_text("bad-b64!!", "k"),
            Err(EncryptError::Base64Decode)
        ));
    }

    #[test]
    fn uses_aes_gcm_not_cbc() {
        // GCM nonces are 12 bytes → 16 chars base64; payload must not look like old IV-only CBC.
        let enc = encrypt_text("x", "k").unwrap();
        let (iv_part, _) = enc.split_once('-').unwrap();
        let nonce = B64.decode(iv_part).unwrap();
        assert_eq!(nonce.len(), NONCE_LEN);
    }
}
