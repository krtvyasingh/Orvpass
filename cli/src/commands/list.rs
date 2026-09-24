#![allow(dead_code)]
use orvpass_core::models::{ItemData, VaultItem};

pub fn execute(json: bool, category: Option<String>) {
    let items = crate::vault::database::load_items();
    if json {
        println!("{}", serde_json::to_string_pretty(&items).unwrap_or_default());
        return;
    }

    if items.is_empty() {
        println!("\x1b[38;2;148;163;184m📦 Vault is empty. Add your first item with: \x1b[1;38;2;129;140;248morvpass quick-add <name> <user> <pass>\x1b[0m");
        return;
    }

    let filtered: Vec<&VaultItem> = if let Some(cat) = category {
        items.iter().filter(|i| i.item_type.to_string().eq_ignore_ascii_case(&cat)).collect()
    } else {
        items.iter().collect()
    };

    println!("\x1b[1;38;2;129;140;248m┌── 🛡️  ORVPASS VAULT ────────────────────────────────────────────────────────┐\x1b[0m");
    println!("\x1b[1;38;2;129;140;248m│\x1b[0m  \x1b[1;38;2;255;255;255m{:<4} {:<24} {:<28} {:<10}\x1b[0m \x1b[1;38;2;129;140;248m│\x1b[0m", "TYPE", "TITLE", "USERNAME / IDENTITY", "FAVORITE");
    println!("\x1b[1;38;2;129;140;248m├───┼────────────────────────┼──────────────────────────────┼──────────┤\x1b[0m");

    for item in &filtered {
        let (icon, user) = match &item.data {
            ItemData::Login(l) => ("🔑", l.username.as_deref().unwrap_or("—")),
            ItemData::SecureNote(_) => ("📝", "[Secure Note]"),
            ItemData::CreditCard(_) => ("💳", "•••• 4242"),
            _ => ("📦", "—"),
        };
        let is_fav = if item.tags.iter().any(|t| t == "favorite" || t == "pinned") { "⭐ Yes" } else { "  No " };

        println!("\x1b[1;38;2;129;140;248m│\x1b[0m  {:<2} {:<24} {:<28} {:<10} \x1b[1;38;2;129;140;248m│\x1b[0m",
            icon,
            item.title.chars().take(22).collect::<String>(),
            user.chars().take(26).collect::<String>(),
            is_fav
        );
    }
    println!("\x1b[1;38;2;129;140;248m└───┴────────────────────────┴──────────────────────────────┴──────────┘\x1b[0m");
    println!("\x1b[38;2;148;163;184mTotal: {} credential(s) | Press \x1b[1;38;2;129;140;248morvpass\x1b[0m \x1b[38;2;148;163;184mfor interactive TUI dashboard\x1b[0m", filtered.len());
}

pub fn render_compact_list(items: &[VaultItem]) {
    for item in items {
        println!("  ▶ 🔑 {}", item.title);
    }
}
