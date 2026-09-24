use crate::models::{ItemData, VaultItem};

pub fn export_aegis_json(items: &[VaultItem]) -> String {
    let mut entries = Vec::new();
    for item in items {
        if let ItemData::Login(l) = &item.data {
            entries.push(serde_json::json!({
                "type": "totp",
                "name": item.title,
                "issuer": "Orvpass",
                "info": {
                    "secret": l.password.clone().unwrap_or_else(|| "SECRET".into()),
                    "algo": "SHA1",
                    "digits": 6,
                    "period": 30
                }
            }));
        }
    }
    serde_json::to_string_pretty(&serde_json::json!({
        "version": 1,
        "db": { "entries": entries }
    })).unwrap_or_default()
}
