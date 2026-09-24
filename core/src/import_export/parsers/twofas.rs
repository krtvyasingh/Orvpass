use crate::models::{ItemData, ItemType, LoginData, VaultItem};

pub fn parse_2fas_json(content: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(content) {
        if let Some(services) = v.get("services").and_then(|s| s.as_array()) {
            for s in services {
                let name = s.get("name").and_then(|n| n.as_str()).unwrap_or("2FAS Service");
                let secret = s.get("secret").and_then(|sec| sec.as_str()).unwrap_or("SECRET");
                items.push(VaultItem::new(
                    ItemType::Totp,
                    name,
                    ItemData::Login(LoginData { username: Some(name.into()), password: Some(secret.into()), urls: vec![] })
                ));
            }
        }
    }
    items
}
