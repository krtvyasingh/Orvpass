use orvpass_core::models::{VaultItem, ItemType, ItemData, LoginData};
use orvpass_core::import_export::exporters::proton::export_proton_pass_json;
use orvpass_core::import_export::exporters::dashlane::export_dashlane_json;
use orvpass_core::import_export::exporters::aws_sm::export_aws_secrets_json;
use orvpass_core::import_export::exporters::gcp_sm::export_gcp_secrets_json;

#[test]
fn test_proton_exporter() {
    let items = vec![VaultItem::new(ItemType::Login, "ProtonAccount", ItemData::Login(LoginData {
        username: Some("user@proton.me".into()),
        password: Some("secret123".into()),
        urls: vec!["https://mail.proton.me".into()]
    }))];
    let res = export_proton_pass_json(&items);
    assert!(res.contains("ProtonAccount"));
    assert!(res.contains("user@proton.me"));
}

#[test]
fn test_dashlane_exporter() {
    let items = vec![VaultItem::new(ItemType::Login, "DashlaneAcc", ItemData::Login(LoginData {
        username: Some("dashuser".into()),
        password: Some("dashpass".into()),
        urls: vec![]
    }))];
    let res = export_dashlane_json(&items);
    assert!(res.contains("AUTHENTIFIANT"));
}

#[test]
fn test_aws_gcp_exporters() {
    let items = vec![VaultItem::new(ItemType::Login, "PROD_API_KEY", ItemData::Login(LoginData {
        username: None,
        password: Some("aws-secret-val".into()),
        urls: vec![]
    }))];
    let aws_res = export_aws_secrets_json(&items);
    assert!(aws_res.contains("PROD_API_KEY"));
    assert!(aws_res.contains("aws-secret-val"));

    let gcp_res = export_gcp_secrets_json(&items);
    assert!(gcp_res.contains("PROD_API_KEY"));
}
