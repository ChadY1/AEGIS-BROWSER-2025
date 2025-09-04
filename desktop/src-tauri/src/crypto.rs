use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString};
use chacha20poly1305::{aead::{Aead, KeyInit}, XChaCha20Poly1305, XNonce};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use anyhow::Result;

pub struct KeyMaterial { pub key: [u8; 32] }

pub fn derive_key(weekly_pass: &str, week_tag: &str) -> KeyMaterial {
    // week_tag = YYYY-WW
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let hash = argon.hash_password((weekly_pass.to_owned()+week_tag).as_bytes(), &salt).unwrap();
    let key_bytes = Sha256::digest(hash.to_string().as_bytes());
    let mut key = [0u8;32];
    key.copy_from_slice(&key_bytes[..32]);
    KeyMaterial { key }
}

pub fn encrypt(key: &KeyMaterial, plaintext: &[u8]) -> Result<Vec<u8>> {
    let cipher = XChaCha20Poly1305::new((&key.key).into());
    let nonce = XNonce::from(rand::random::<[u8;24]>());
    let mut out = nonce.to_vec();
    let mut ct = cipher.encrypt(&nonce, plaintext)?;
    out.append(&mut ct);
    Ok(out)
}

pub fn decrypt(key: &KeyMaterial, data: &[u8]) -> Result<Vec<u8>> {
    let cipher = XChaCha20Poly1305::new((&key.key).into());
    let (nonce, ct) = data.split_at(24);
    Ok(cipher.decrypt(nonce.into(), ct)?)
}
