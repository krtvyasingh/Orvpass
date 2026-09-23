use crate::models::{ItemData, VaultItem};

pub fn export_proton_pass_json(items: &[VaultItem]) -> String {
    let mut proton_items = Vec::new();
    for item in items {
        if let ItemData::Login(l) = &item.data {
            proton_items.push(serde_json::json!({
                "itemId": item.id,
                "data": {
                    "metadata": { "name": item.title, "note": "" },
                    "content": {
                        "username": l.username,
                        "password": l.password,
                        "urls": l.urls
                    }
                }
            }));
        }
    }
    serde_json::to_string_pretty(&serde_json::json!({
        "vault": { "items": proton_items }
    })).unwrap_or_default()
}
