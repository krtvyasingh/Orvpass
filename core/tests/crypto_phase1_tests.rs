use orvpass_core::mnemonic::{generate_24_word_mnemonic, validate_mnemonic};
use orvpass_core::sss::{split_secret, recover_secret};
use orvpass_core::crypto::{calibrate_argon2_params, EnvelopeV3, generate_salt};

#[test]
fn test_mnemonic_roundtrip() {
    let words = generate_24_word_mnemonic();
    assert_eq!(words.len(), 24);
    assert!(validate_mnemonic(&words));
}

#[test]
fn test_sss_roundtrip() {
    let secret = b"TOP_SECRET_MASTER_KEY_2026";
    let shards = split_secret(secret, 3, 5);
    assert_eq!(shards.len(), 5);
    let recovered = recover_secret(&shards[0..3]).expect("recovery failed");
    assert_eq!(recovered, secret);
}

#[test]
fn test_envelope_v3_creation() {
    let salt = generate_salt();
    let nonce = [1u8; 12];
    let env = EnvelopeV3::new(salt, nonce);
    assert_eq!(&env.magic, b"ORVP");
    assert_eq!(env.version, 3);
    assert_eq!(env.cipher_id, 1);
}

#[test]
fn test_argon2_calibration() {
    let (m, t, p) = calibrate_argon2_params(500);
    assert!(m >= 64 * 1024);
    assert!(t >= 3);
    assert_eq!(p, 4);
}
