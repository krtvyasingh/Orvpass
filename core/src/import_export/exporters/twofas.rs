use crate::models::{ItemData, VaultItem};

pub fn export_2fas_json(items: &[VaultItem]) -> String {
    let mut services = Vec::new();
    for item in items {
        if let ItemData::Login(l) = &item.data {
            services.push(serde_json::json!({
                "name": item.title,
                "secret": l.password.clone().unwrap_or_else(|| "SECRET".into()),
                "tokenType": "TOTP"
            }));
        }
    }
    serde_json::to_string_pretty(&serde_json::json!({
        "schemaVersion": 4,
        "services": services
    })).unwrap_or_default()
}
