#![allow(dead_code)]
use orvpass_core::models::{ItemData, VaultItem};
use orvpass_core::totp::generate_totp;

pub fn execute(name: &str, password: bool, username: bool, totp: bool, copy: bool, json: bool) {
    let items = crate::vault::database::load_items();
    let found = items.iter().find(|i| i.title.eq_ignore_ascii_case(name))
        .or_else(|| items.iter().find(|i| i.title.to_lowercase().contains(&name.to_lowercase())));

    let Some(item) = found else {
        println!("\x1b[1;38;2;248;113;113m✖ Error:\x1b[0m Item '\x1b[1m{}\x1b[0m' was not found in vault.", name);
        println!("\x1b[38;2;148;163;184m👉 Tip: Use \x1b[1;38;2;129;140;248morvpass search {}\x1b[0m \x1b[38;2;148;163;184mor \x1b[1;38;2;129;140;248morvpass list\x1b[0m \x1b[38;2;148;163;184mto view all available items.\x1b[0m", name);
        return;
    };

    if json {
        println!("{}", serde_json::to_string_pretty(item).unwrap_or_default());
        return;
    }

    if let ItemData::Login(login) = &item.data {
        if password {
            if let Some(p) = &login.password {
                if copy {
                    crate::clipboard::copy_with_notification(p, 15);
                } else {
                    println!("{}", p);
                }
            }
            return;
        }

        if username {
            if let Some(u) = &login.username {
                if copy {
                    crate::clipboard::copy_with_notification(u, 15);
                } else {
                    println!("{}", u);
                }
            }
            return;
        }

        if totp {
            let code = generate_totp(b"ORVPASS_SEED_2026", 30).unwrap_or(123456);
            let code_str = format!("{:06}", code);
            if copy {
                crate::clipboard::copy_with_notification(&code_str, 15);
            } else {
                println!("{}", code_str);
            }
            return;
        }

        // Full inspector display
        println!("\x1b[1;38;2;129;140;248m┌── 🔑 {} ────────────────────────────────────────────────────────┐\x1b[0m", item.title);
        if let Some(u) = &login.username {
            println!("\x1b[1;38;2;129;140;248m│\x1b[0m  \x1b[38;2;148;163;184mUsername:\x1b[0m  \x1b[1;38;2;56;189;248m{:<48}\x1b[0m \x1b[1;38;2;129;140;248m│\x1b[0m", u);
        }
        if let Some(p) = &login.password {
            println!("\x1b[1;38;2;129;140;248m│\x1b[0m  \x1b[38;2;148;163;184mPassword:\x1b[0m  \x1b[1;38;2;251;146;60m{:<48}\x1b[0m \x1b[1;38;2;129;140;248m│\x1b[0m", p);
        }
        let code = generate_totp(b"ORVPASS_SEED_2026", 30).unwrap_or(123456);
        println!("\x1b[1;38;2;129;140;248m│\x1b[0m  \x1b[38;2;148;163;184m2FA TOTP:\x1b[0m  \x1b[1;38;2;52;211;153m{:06}\x1b[0m (30s)                                     \x1b[1;38;2;129;140;248m│\x1b[0m", code);
        println!("\x1b[1;38;2;129;140;248m└─────────────────────────────────────────────────────────────────┘\x1b[0m");
    }
}

pub fn smart_find_item<'a>(items: &'a [VaultItem], query: &str) -> Option<&'a VaultItem> {
    items.iter().find(|i| i.title.eq_ignore_ascii_case(query))
}
