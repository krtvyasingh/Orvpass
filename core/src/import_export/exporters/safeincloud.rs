use crate::models::{ItemData, VaultItem};

pub fn export_safeincloud_xml(items: &[VaultItem]) -> String {
    let mut cards = String::new();
    for item in items {
        if let ItemData::Login(l) = &item.data {
            cards.push_str(&format!("  <card title=\"{}\" login=\"{}\" password=\"{}\"/>\n",
                item.title,
                l.username.as_deref().unwrap_or(""),
                l.password.as_deref().unwrap_or("")
            ));
        }
    }
    format!("<safeincloud>\n{}</safeincloud>", cards)
}
