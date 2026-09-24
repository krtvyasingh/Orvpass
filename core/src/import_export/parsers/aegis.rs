use crate::models::{ItemData, ItemType, LoginData, VaultItem};

pub fn parse_aegis_json(content: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(content) {
        if let Some(entries) = v.get("db").and_then(|d| d.get("entries")).and_then(|e| e.as_array()) {
            for entry in entries {
                let name = entry.get("name").and_then(|n| n.as_str()).unwrap_or("2FA Account");
                let issuer = entry.get("issuer").and_then(|i| i.as_str()).unwrap_or("Issuer");
                let info = entry.get("info").and_then(|i| i.get("secret")).and_then(|s| s.as_str()).unwrap_or("SECRET");
                let title = format!("{}: {}", issuer, name);
                items.push(VaultItem::new(
                    ItemType::Totp,
                    &title,
                    ItemData::Login(LoginData { username: Some(name.into()), password: Some(info.into()), urls: vec![] })
                ));
            }
        }
    }
    items
}
