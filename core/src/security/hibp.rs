use sha2::{Sha256, Digest};

pub fn compute_sha256_k_anonymity_prefix(secret: &str) -> (String, String) {
    let mut hasher = Sha256::new();
    hasher.update(secret.as_bytes());
    let hash = hex::encode(hasher.finalize()).to_uppercase();
    let prefix = hash[0..5].to_string();
    let suffix = hash[5..].to_string();
    (prefix, suffix)
}
