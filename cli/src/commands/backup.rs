#![allow(dead_code)]
use crate::vault::database;
use sha2::{Sha256, Digest};
use std::fs;
use std::path::Path;

pub fn execute_backup(output_path: Option<String>) {
    let items = database::load_items();
    let json_bytes = serde_json::to_vec_pretty(&items).unwrap_or_default();
    
    let default_name = format!("orvpass_backup_{}.json", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
    let target = output_path.unwrap_or(default_name);
    
    if let Err(e) = fs::write(&target, &json_bytes) {
        println!("\x1b[1;38;2;248;113;113m✖ Error creating backup:\x1b[0m {}", e);
        return;
    }
    
    let mut hasher = Sha256::new();
    hasher.update(&json_bytes);
    let hash_hex = hex::encode(hasher.finalize());
    
    println!("\x1b[1;38;2;52;211;153m✔ Vault snapshot backup created successfully!\x1b[0m");
    println!("  📁 File:     {}", target);
    println!("  📦 Items:    {}", items.len());
    println!("  🔒 SHA-256:  {}", hash_hex);
}

pub fn execute_restore(input_path: &str) {
    let path = Path::new(input_path);
    if !path.exists() {
        println!("\x1b[1;38;2;248;113;113m✖ Error:\x1b[0m File '{}' does not exist.", input_path);
        return;
    }
    
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            println!("\x1b[1;38;2;248;113;113m✖ Failed to read backup file:\x1b[0m {}", e);
            return;
        }
    };
    
    match serde_json::from_slice::<Vec<orvpass_core::models::VaultItem>>(&bytes) {
        Ok(items) => {
            let count = items.len();
            if let Err(e) = database::save_items(&items) {
                println!("\x1b[1;38;2;248;113;113m✖ Failed to restore vault:\x1b[0m {}", e);
            } else {
                println!("\x1b[1;38;2;52;211;153m✔ Vault restored successfully!\x1b[0m Total {} item(s) loaded.", count);
            }
        }
        Err(e) => {
            println!("\x1b[1;38;2;248;113;113m✖ Invalid backup file format:\x1b[0m {}", e);
        }
    }
}
