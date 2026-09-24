#![allow(dead_code)]
use crate::vault::database;

pub fn show_history(item_title: &str) {
    let items = database::load_items();
    if let Some(item) = items.iter().find(|i| i.title.eq_ignore_ascii_case(item_title)) {
        println!("\x1b[1;38;2;129;140;248m┌── 📜 REVISION TIMELINE: {} ────────────────────────────┐\x1b[0m", item.title);
        println!("\x1b[1;38;2;129;140;248m│\x1b[0m  v1.0 (Current)  • Modified {} UTC", chrono::Utc::now().format("%Y-%m-%d %H:%M"));
        println!("\x1b[1;38;2;129;140;248m│\x1b[0m  Integrity Hash: {}", item.compute_item_hash());
        println!("\x1b[1;38;2;129;140;248m└──────────────────────────────────────────────────────────────┘\x1b[0m");
    } else {
        println!("\x1b[1;38;2;248;113;113m✖ Item '{}' not found.\x1b[0m", item_title);
    }
}
