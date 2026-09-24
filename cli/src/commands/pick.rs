#![allow(dead_code)]
use crate::vault::database;
use orvpass_core::models::ItemData;
use dialoguer::{Select, theme::ColorfulTheme};

pub fn execute_pick() {
    let items = database::load_items();
    if items.is_empty() {
        println!("\x1b[38;2;148;163;184m📦 Vault is empty.\x1b[0m");
        return;
    }

    let titles: Vec<String> = items.iter().map(|i| {
        let icon = match i.data {
            ItemData::Login(_) => "🔑",
            ItemData::SecureNote(_) => "📝",
            ItemData::CreditCard(_) => "💳",
            _ => "📦",
        };
        format!("{} {}", icon, i.title)
    }).collect();

    if let Ok(Some(selection)) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select credential to copy password")
        .default(0)
        .items(&titles[..])
        .interact_opt()
    {
        let item = &items[selection];
        if let ItemData::Login(l) = &item.data {
            if let Some(p) = &l.password {
                crate::clipboard::copy_with_notification(p, 15);
                println!("\x1b[1;38;2;52;211;153m✔ Copied password for '{}' to clipboard (auto-wipes 15s)\x1b[0m", item.title);
                return;
            }
        }
        println!("ℹ Selected item has no password to copy.");
    }
}
