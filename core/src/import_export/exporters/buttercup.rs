use crate::models::{ItemData, VaultItem};

pub fn export_buttercup_json(items: &[VaultItem]) -> String {
    let mut entries = Vec::new();
    for item in items {
        if let ItemData::Login(l) = &item.data {
            entries.push(serde_json::json!({
                "title": item.title,
                "username": l.username,
                "password": l.password
            }));
        }
    }
    serde_json::to_string_pretty(&serde_json::json!({ "entries": entries })).unwrap_or_default()
}
