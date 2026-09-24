use crate::models::{ItemData, ItemType, LoginData, VaultItem};
use super::csv_utils::parse_csv_records;

pub fn parse_nordpass_csv(content: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    let records = parse_csv_records(content);
    for rec in records {
        let title = rec.get("title").or_else(|| rec.get("name")).cloned().unwrap_or_else(|| "NordPass Item".into());
        let username = rec.get("username").or_else(|| rec.get("email")).cloned();
        let password = rec.get("password").cloned();
        let url = rec.get("url").cloned();
        items.push(VaultItem::new(
            ItemType::Login,
            &title,
            ItemData::Login(LoginData { username, password, urls: url.into_iter().collect() })
        ));
    }
    items
}
