//! Base64, UUID, and random alphanumeric helpers (home page tools).

use base64::{engine::general_purpose::STANDARD as B64, Engine};
use rand::RngCore;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CodecError {
    #[error("invalid base64")]
    InvalidBase64,
    #[error("invalid utf-8 in decoded data")]
    InvalidUtf8,
}

pub fn encode_base64(text: &str) -> String {
    B64.encode(text.as_bytes())
}

pub fn decode_base64(text: &str) -> Result<String, CodecError> {
    let bytes = B64
        .decode(text.trim())
        .map_err(|_| CodecError::InvalidBase64)?;
    String::from_utf8(bytes).map_err(|_| CodecError::InvalidUtf8)
}

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn generate_alphanumeric(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::rng();
    let mut out = String::with_capacity(length);
    for _ in 0..length {
        let idx = (rng.next_u32() as usize) % CHARSET.len();
        out.push(CHARSET[idx] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_roundtrip() {
        let enc = encode_base64("hello");
        assert_eq!(decode_base64(&enc).unwrap(), "hello");
    }

    #[test]
    fn uuid_format() {
        let id = generate_uuid();
        assert_eq!(id.len(), 36);
        assert!(id.contains('-'));
    }

    #[test]
    fn alphanumeric_length() {
        let s = generate_alphanumeric(32);
        assert_eq!(s.len(), 32);
    }
}
