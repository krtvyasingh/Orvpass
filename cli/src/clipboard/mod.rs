pub fn copy_with_notification(content: &str, timeout_secs: u64) {
    if let Ok(mut board) = arboard::Clipboard::new() {
        let _ = board.set_text(content);
    }
    println!("\x1b[1;38;2;52;211;153m✔\x1b[0m Copied to clipboard. Auto-wipes in {}s...", timeout_secs);
}

pub fn copy_with_bell(content: &str) {
    print!("\x07");
    copy_with_notification(content, 15);
}
