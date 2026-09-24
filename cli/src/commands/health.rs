#![allow(dead_code)]
use crate::vault::database;
use orvpass_core::models::ItemData;
use orvpass_core::security::password_analyzer::analyze_password;

pub fn execute_health() {
    let items = database::load_items();
    if items.is_empty() {
        println!("\x1b[38;2;148;163;184m📦 Vault is empty. No health metrics to compute.\x1b[0m");
        return;
    }

    let mut total_logins = 0;
    let mut weak_passwords = 0;
    let mut strong_passwords = 0;
    let mut passwords_set = std::collections::HashSet::new();
    let mut reused_passwords = 0;
    let mut with_2fa = 0;

    for item in &items {
        if let ItemData::Login(l) = &item.data {
            total_logins += 1;
            if let Some(pass) = &l.password {
                if !passwords_set.insert(pass.clone()) {
                    reused_passwords += 1;
                }
                let report = analyze_password(pass);
                if report.score < 2 {
                    weak_passwords += 1;
                } else if report.score >= 3 {
                    strong_passwords += 1;
                }
            }
            if item.tags.iter().any(|t| t == "2fa" || t == "totp") {
                with_2fa += 1;
            }
        }
    }

    let base_score: f64 = 100.0;
    let weak_penalty = (weak_passwords as f64) * 15.0;
    let reuse_penalty = (reused_passwords as f64) * 20.0;
    let final_score = (base_score - weak_penalty - reuse_penalty).clamp(10.0, 100.0) as u32;

    let grade = match final_score {
        90..=100 => "A+ (Excellent)",
        75..=89 => "B (Good)",
        50..=74 => "C (Needs Attention)",
        _ => "F (Critical Vulnerabilities)",
    };

    println!("\x1b[1;38;2;129;140;248m┌── 🛡️  VAULT SECURITY HEALTH REPORT ────────────────────────────────────────┐\x1b[0m");
    println!("\x1b[1;38;2;129;140;248m│\x1b[0m  \x1b[1;38;2;255;255;255mOVERALL HEALTH SCORE:  \x1b[1;38;2;52;211;153m{}/100 [{}]\x1b[0m", final_score, grade);
    println!("\x1b[1;38;2;129;140;248m├─────────────────────────────────────────────────────────────────────────────┤\x1b[0m");
    println!("\x1b[1;38;2;129;140;248m│\x1b[0m  📦 Total Vault Items:      \x1b[1m{:<48}\x1b[0m \x1b[1;38;2;129;140;248m│\x1b[0m", items.len());
    println!("\x1b[1;38;2;129;140;248m│\x1b[0m  🔑 Login Credentials:      \x1b[1m{:<48}\x1b[0m \x1b[1;38;2;129;140;248m│\x1b[0m", total_logins);
    println!("\x1b[1;38;2;129;140;248m│\x1b[0m  💪 Strong Passwords:       \x1b[38;2;52;211;153m{:<48}\x1b[0m \x1b[1;38;2;129;140;248m│\x1b[0m", strong_passwords);
    println!("\x1b[1;38;2;129;140;248m│\x1b[0m  ⚠️ Weak Passwords:         \x1b[38;2;251;191;36m{:<48}\x1b[0m \x1b[1;38;2;129;140;248m│\x1b[0m", weak_passwords);
    println!("\x1b[1;38;2;129;140;248m│\x1b[0m  🔁 Reused Passwords:       \x1b[38;2;248;113;113m{:<48}\x1b[0m \x1b[1;38;2;129;140;248m│\x1b[0m", reused_passwords);
    println!("\x1b[1;38;2;129;140;248m│\x1b[0m  🛡️ 2FA Protection:         \x1b[38;2;56;189;248m{:<48}\x1b[0m \x1b[1;38;2;129;140;248m│\x1b[0m", format!("{} item(s) protected", with_2fa));
    println!("\x1b[1;38;2;129;140;248m└─────────────────────────────────────────────────────────────────────────────┘\x1b[0m");
}
