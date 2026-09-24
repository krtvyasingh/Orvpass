use crate::models::{ItemData, VaultItem};

pub fn export_keeper_json(items: &[VaultItem]) -> String {
    let mut list = Vec::new();
    for item in items {
        if let ItemData::Login(l) = &item.data {
            list.push(serde_json::json!({
                "title": item.title,
                "login": l.username,
                "password": l.password,
                "login_url": l.urls.first()
            }));
        }
    }
    serde_json::to_string_pretty(&serde_json::json!({ "records": list })).unwrap_or_default()
}
