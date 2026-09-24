#![allow(dead_code)]
// High-Aesthetic CLI Formatter & Color Palette for Orvpass
pub fn banner() {
    println!("\x1b[38;2;99;102;241m┌──────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("\x1b[38;2;99;102;241m│  \x1b[1;38;2;255;255;255m⚡ ORVPASS ENTERPRISE v5.5.0\x1b[0m \x1b[38;2;129;140;248m[Zero-Knowledge Memory-Safe]\x1b[0m  \x1b[38;2;99;102;241m│\x1b[0m");
    println!("\x1b[38;2;99;102;241m└──────────────────────────────────────────────────────────────┘\x1b[0m");
}

pub fn success(msg: &str) {
    println!("\x1b[1;38;2;52;211;153m✔\x1b[0m \x1b[38;2;226;232;240m{}\x1b[0m", msg);
}

pub fn info(msg: &str) {
    println!("\x1b[1;38;2;56;189;248mℹ\x1b[0m \x1b[38;2;203;213;225m{}\x1b[0m", msg);
}

pub fn warn(msg: &str) {
    println!("\x1b[1;38;2;251;191;36m⚠\x1b[0m \x1b[38;2;254;240;138m{}\x1b[0m", msg);
}

pub fn error(msg: &str) {
    println!("\x1b[1;38;2;248;113;113m✖\x1b[0m \x1b[38;2;254;202;202m{}\x1b[0m", msg);
}

pub fn print_highlighted_json(json_str: &str) {
    println!("{}", json_str);
}
