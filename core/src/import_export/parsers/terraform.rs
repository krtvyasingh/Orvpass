use crate::models::{ItemData, ItemType, LoginData, VaultItem};

pub fn parse_tfvars_json(content: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(content) {
        if let Some(obj) = v.as_object() {
            for (key, val) in obj {
                let val_str = if let Some(s) = val.as_str() { s.to_string() } else { val.to_string() };
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
