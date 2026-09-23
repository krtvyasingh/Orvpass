use orvpass_core::mnemonic::generate_24_word_mnemonic;
use orvpass_core::crypto::calibrate_argon2_params;

#[test]
fn test_mnemonic_generation_24_words() {
    let words = generate_24_word_mnemonic();
    assert_eq!(words.len(), 24);
}

#[test]
fn test_calibrate_argon2() {
    let (m, t, p) = calibrate_argon2_params(500);
    assert_eq!(m, 65536);
    assert_eq!(t, 3);
    assert_eq!(p, 4);
}
