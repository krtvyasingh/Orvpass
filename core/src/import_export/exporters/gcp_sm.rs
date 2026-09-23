use crate::models::{ItemData, VaultItem};

pub fn export_gcp_secrets_json(items: &[VaultItem]) -> String {
    let mut list = Vec::new();
    for item in items {
        if let ItemData::Login(l) = &item.data {
            list.push(serde_json::json!({
                "name": item.title,
                "payload": {
                    "data": l.password.clone().unwrap_or_default()
                }
            }));
        }
    }
    serde_json::to_string_pretty(&serde_json::json!({ "secrets": list })).unwrap_or_default()
}
