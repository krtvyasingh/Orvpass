#![allow(dead_code)]
use crate::vault::database;
use std::fs;
use std::path::Path;
use base64::{Engine, engine::general_purpose};

pub fn attach_file(item_title: &str, file_path: &str) {
    let path = Path::new(file_path);
    if !path.exists() {
        println!("\x1b[1;38;2;248;113;113m✖ Error:\x1b[0m File '{}' does not exist.", file_path);
        return;
    }

    let file_bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            println!("\x1b[1;38;2;248;113;113m✖ Error reading file:\x1b[0m {}", e);
            return;
        }
    };

    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("attachment.bin");
    let encoded = general_purpose::STANDARD.encode(&file_bytes);

    let mut items = database::load_items();
    if let Some(item) = items.iter_mut().find(|i| i.title.eq_ignore_ascii_case(item_title)) {
        item.add_custom_field(&format!("attachment:{}", filename), &encoded, true);
        let _ = database::save_items(&items);
        println!("\x1b[1;38;2;52;211;153m✔ Successfully attached '{}' ({} bytes) to credential '{}'!\x1b[0m", filename, file_bytes.len(), item.title);
    } else {
        println!("\x1b[1;38;2;248;113;113m✖ Credential '{}' not found in vault.\x1b[0m", item_title);
    }
}

pub fn extract_file(item_title: &str, attachment_name: &str, output_path: Option<String>) {
    let items = database::load_items();
    if let Some(item) = items.iter().find(|i| i.title.eq_ignore_ascii_case(item_title)) {
        let field_key = format!("attachment:{}", attachment_name);
        if let Some(field) = item.custom_fields.iter().find(|f| f.name.eq_ignore_ascii_case(&field_key) || f.name.eq_ignore_ascii_case(attachment_name)) {
            if let Ok(bytes) = general_purpose::STANDARD.decode(&field.value) {
                let target = output_path.unwrap_or_else(|| attachment_name.to_string());
                if let Err(e) = fs::write(&target, &bytes) {
                    println!("\x1b[1;38;2;248;113;113m✖ Failed to write extracted file:\x1b[0m {}", e);
                } else {
                    println!("\x1b[1;38;2;52;211;153m✔ Extracted attachment to '{}' ({} bytes)\x1b[0m", target, bytes.len());
                }
                return;
            }
        }
        println!("\x1b[1;38;2;248;113;113m✖ Attachment '{}' not found on credential '{}'.\x1b[0m", attachment_name, item_title);
    } else {
        println!("\x1b[1;38;2;248;113;113m✖ Credential '{}' not found in vault.\x1b[0m", item_title);
    }
}
