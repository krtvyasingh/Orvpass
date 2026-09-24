use crate::models::{ItemData, ItemType, LoginData, VaultItem};
use super::csv_utils::parse_csv_records;

pub fn parse_zoho_csv(content: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    let records = parse_csv_records(content);
    for rec in records {
        let title = rec.get("secret_name").or_else(|| rec.get("name")).cloned().unwrap_or_else(|| "Zoho Secret".into());
        let username = rec.get("user_name").or_else(|| rec.get("username")).cloned();
        let password = rec.get("password").cloned();
        let url = rec.get("secret_url").or_else(|| rec.get("url")).cloned();
        items.push(VaultItem::new(
            ItemType::Login,
            &title,
            ItemData::Login(LoginData { username, password, urls: url.into_iter().collect() })
        ));
    }
    items
}
