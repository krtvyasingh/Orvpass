use crate::models::{ItemData, VaultItem};

pub fn export_dashlane_json(items: &[VaultItem]) -> String {
    let mut d_items = Vec::new();
    for item in items {
        if let ItemData::Login(l) = &item.data {
            d_items.push(serde_json::json!({
                "title": item.title,
                "login": l.username,
                "password": l.password,
                "url": l.urls.first().cloned().unwrap_or_default()
            }));
        }
    }
    serde_json::to_string_pretty(&serde_json::json!({
        "AUTHENTIFIANT": d_items
    })).unwrap_or_default()
}
