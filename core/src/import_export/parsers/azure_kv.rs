use crate::models::{ItemData, ItemType, LoginData, VaultItem};

pub fn parse_azure_kv_json(content: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(content) {
        if let Some(obj) = v.as_object() {
            for (key, val) in obj {
                let val_str = val.as_str().unwrap_or("").to_string();
                items.push(VaultItem::new(
                    ItemType::Login,
                    key,
                    ItemData::Login(LoginData { username: None, password: Some(val_str), urls: vec![] })
                ));
            }
        }
    }
    items
}
