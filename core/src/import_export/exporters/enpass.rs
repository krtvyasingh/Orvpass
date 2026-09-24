use crate::models::{ItemData, VaultItem};

pub fn export_enpass_json(items: &[VaultItem]) -> String {
    let mut list = Vec::new();
    for item in items {
        if let ItemData::Login(l) = &item.data {
            list.push(serde_json::json!({
                "title": item.title,
                "fields": [
                    { "label": "Username", "value": l.username },
                    { "label": "Password", "value": l.password }
                ]
            }));
        }
    }
    serde_json::to_string_pretty(&serde_json::json!({ "items": list })).unwrap_or_default()
}
