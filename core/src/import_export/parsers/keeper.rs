use crate::models::{ItemData, ItemType, LoginData, VaultItem};
use super::csv_utils::parse_csv_records;

pub fn parse_keeper_csv(content: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    let records = parse_csv_records(content);
    for rec in records {
        let title = rec.get("title").or_else(|| rec.get("folder")).cloned().unwrap_or_else(|| "Keeper Item".into());
        let username = rec.get("login").or_else(|| rec.get("username")).cloned();
        let password = rec.get("password").cloned();
        let url = rec.get("url").or_else(|| rec.get("login_url")).cloned();
        items.push(VaultItem::new(
            ItemType::Login,
            &title,
            ItemData::Login(LoginData { username, password, urls: url.into_iter().collect() })
        ));
    }
    items
}
