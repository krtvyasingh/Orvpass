use orvpass_core::security::hibp::compute_sha256_k_anonymity_prefix;
use orvpass_core::security::password_analyzer::analyze_password;

#[test]
fn test_hibp_k_anonymity_prefix() {
    let (prefix, suffix) = compute_sha256_k_anonymity_prefix("password123");
    assert_eq!(prefix.len(), 5);
    assert_eq!(suffix.len(), 59); // 64 - 5
}

#[test]
fn test_password_strength_analyzer() {
    let weak = analyze_password("12345");
    assert_eq!(weak.score, 0);
    assert!(weak.is_compromised_risk);

    let strong = analyze_password("correct-horse-battery-staple-99$#");
    assert!(strong.entropy_bits > 80.0);
    assert!(strong.score >= 3);
    assert!(!strong.is_compromised_risk);
}
