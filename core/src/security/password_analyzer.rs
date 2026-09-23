pub struct PasswordStrengthReport {
    pub entropy_bits: f64,
    pub score: u8, // 0 to 4
    pub crack_time_display: String,
    pub is_compromised_risk: bool,
}

pub fn analyze_password(password: &str) -> PasswordStrengthReport {
    let len = password.len();
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_ascii_alphanumeric());

    let mut pool_size: usize = 0;
    if has_lower { pool_size += 26; }
    if has_upper { pool_size += 26; }
    if has_digit { pool_size += 10; }
    if has_special { pool_size += 32; }
    if pool_size == 0 { pool_size = 1; }

    let entropy = (len as f64) * (pool_size as f64).log2();
    let score = match entropy as usize {
        0..=35 => 0,
        36..=59 => 1,
        60..=79 => 2,
        80..=99 => 3,
        _ => 4,
    };

    let crack_time = match score {
        0 => "Instant (< 1 ms)",
        1 => "Minutes to Hours",
        2 => "Several Months",
        3 => "Decades",
        _ => "Centuries / Uncrackable",
    }.to_string();

    PasswordStrengthReport {
        entropy_bits: entropy,
        score,
        crack_time_display: crack_time,
        is_compromised_risk: score < 2,
    }
}
