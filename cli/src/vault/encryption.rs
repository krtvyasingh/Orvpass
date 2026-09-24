use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, KeyInit},
};

use argon2::Argon2;
use rand::TryRngCore;
use rand::rngs::OsRng;
use base64::{Engine, engine::general_purpose};

const VERSION: &str = "ORVPASS1";

pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .expect("argon2 failed");
    key
}

pub fn encrypt(plaintext: &str, password: &str) -> String {
    let mut salt = [0u8; 16];
    OsRng.try_fill_bytes(&mut salt).expect("salt generation failed");

    let key = derive_key(password, &salt);
    let cipher = Aes256Gcm::new_from_slice(&key).expect("valid key length");

    let mut nonce = [0u8; 12];
    OsRng.try_fill_bytes(&mut nonce).expect("nonce generation failed");

    let encrypted = cipher
        .encrypt((&nonce).into(), plaintext.as_bytes())
        .expect("encrypt failed");

    format!(
        "{}:{}:{}:{}",
        VERSION,
        general_purpose::STANDARD.encode(salt),
        general_purpose::STANDARD.encode(nonce),
        general_purpose::STANDARD.encode(encrypted)
    )
}

pub fn decrypt(data: &str, password: &str) -> Option<String> {
    let parts: Vec<&str> = data.split(':').collect();
    if parts.len() != 4 || parts[0] != VERSION {
        return None;
    }

    let salt = general_purpose::STANDARD.decode(parts[1]).ok()?;
    let nonce = general_purpose::STANDARD.decode(parts[2]).ok()?;
    let encrypted = general_purpose::STANDARD.decode(parts[3]).ok()?;

    if nonce.len() != 12 {
        return None;
    }

    let key = derive_key(password, &salt);
    let cipher = Aes256Gcm::new_from_slice(&key).ok()?;
    let nonce_arr: [u8; 12] = nonce.try_into().ok()?;

    let plain = cipher
        .decrypt((&nonce_arr).into(), encrypted.as_ref())
        .ok()?;

    String::from_utf8(plain).ok()
}
