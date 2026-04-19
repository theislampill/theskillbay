use base64::engine::general_purpose::STANDARD as BASE64_ENGINE;
use base64::Engine;
use ring::signature::{Ed25519KeyPair, KeyPair, UnparsedPublicKey, ED25519};
use ring::digest::{Context, SHA256};
use std::fs;
use anyhow::Result;

/// Generate SHA-256 hash of data
pub fn sha256(data: &[u8]) -> String {
    let mut context = Context::new(&SHA256);
    context.update(data);
    let digest = context.finish();
    hex::encode(digest.as_ref())
}

/// Generate Ed25519 key pair
pub fn generate_keypair() -> Result<(String, String)> {
    let rng = ring::rand::SystemRandom::new();
    let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng)?;
    let keypair = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref())?;
    let public_key = hex::encode(keypair.public_key().as_ref());
    let private_key = BASE64_ENGINE.encode(pkcs8.as_ref());
    Ok((public_key, private_key))
}

/// Sign data with private key
pub fn sign(data: &[u8], private_key_b64: &str) -> Result<String> {
    let private_key_bytes = BASE64_ENGINE.decode(private_key_b64)?;
    let keypair = Ed25519KeyPair::from_pkcs8(&private_key_bytes)?;
    let signature = keypair.sign(data);
    Ok(BASE64_ENGINE.encode(signature.as_ref()))
}

/// Verify signature
pub fn verify(data: &[u8], signature_b64: &str, public_key_hex: &str) -> Result<bool> {
    let signature_bytes = BASE64_ENGINE.decode(signature_b64)?;
    let public_key_bytes = hex::decode(public_key_hex)?;
    let public_key = UnparsedPublicKey::new(&ED25519, &public_key_bytes);
    match public_key.verify(data, &signature_bytes) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Check PoW: SHA-256(content_hash + nonce) has at least difficulty leading zeros
pub fn check_pow(content_hash: &str, nonce: &str, difficulty: u32) -> bool {
    let combined = format!("{}{}", content_hash, nonce);
    let hash = sha256(combined.as_bytes());
    let leading_zeros = hash.chars().take_while(|&c| c == '0').count();
    leading_zeros >= difficulty as usize
}

/// Find nonce for PoW
pub fn find_nonce(content_hash: &str, difficulty: u32) -> String {
    let mut nonce = 0;
    loop {
        let combined = format!("{}{}", content_hash, nonce);
        let hash = sha256(combined.as_bytes());
        let leading_zeros = hash.chars().take_while(|&c| c == '0').count();
        if leading_zeros >= difficulty as usize {
            return nonce.to_string();
        }
        nonce += 1;
    }
}