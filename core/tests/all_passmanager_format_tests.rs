use orvpass_core::models::{VaultItem, ItemType, ItemData, LoginData};
use orvpass_core::import_export::parsers::nordpass::parse_nordpass_csv;
use orvpass_core::import_export::exporters::nordpass::export_nordpass_csv;
use orvpass_core::import_export::parsers::keeper::parse_keeper_csv;
use orvpass_core::import_export::exporters::keeper::export_keeper_json;
use orvpass_core::import_export::parsers::zoho::parse_zoho_csv;
use orvpass_core::import_export::exporters::zoho::export_zoho_csv;
use orvpass_core::import_export::parsers::aegis::parse_aegis_json;
use orvpass_core::import_export::exporters::aegis::export_aegis_json;
use orvpass_core::import_export::parsers::twofas::parse_2fas_json;
use orvpass_core::import_export::exporters::twofas::export_2fas_json;
use orvpass_core::import_export::parsers::azure_kv::parse_azure_kv_json;
use orvpass_core::import_export::exporters::azure_kv::export_azure_kv_json;
use orvpass_core::import_export::parsers::terraform::parse_tfvars_json;
use orvpass_core::import_export::exporters::terraform::export_tfvars_json;
use orvpass_core::import_export::exporters::safeincloud::export_safeincloud_xml;
use orvpass_core::import_export::exporters::buttercup::export_buttercup_json;
use orvpass_core::import_export::exporters::passbolt::export_passbolt_csv;
use orvpass_core::import_export::exporters::roboform::export_roboform_csv;

#[test]
fn test_nordpass_roundtrip() {
    let csv = "title,username,password,url,note\nMyNordAccount,user@nord.com,nordpass123,https://nord.com,\n";
    let items = parse_nordpass_csv(csv);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].title, "MyNordAccount");
    let exp = export_nordpass_csv(&items);
    assert!(exp.contains("MyNordAccount"));
    assert!(exp.contains("user@nord.com"));
}

#[test]
fn test_keeper_roundtrip() {
    let csv = "title,login,password,url\nKeeperAccount,keeperuser,keeperpass,https://keeper.com\n";
    let items = parse_keeper_csv(csv);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].title, "KeeperAccount");
    let exp = export_keeper_json(&items);
    assert!(exp.contains("KeeperAccount"));
    assert!(exp.contains("keeperuser"));
}

#[test]
fn test_zoho_roundtrip() {
    let csv = "secret_name,user_name,password,secret_url\nZohoVaultItem,zohouser,zohopass,https://zoho.com\n";
    let items = parse_zoho_csv(csv);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].title, "ZohoVaultItem");
    let exp = export_zoho_csv(&items);
    assert!(exp.contains("ZohoVaultItem"));
}

#[test]
fn test_aegis_and_2fas() {
    let sample_aegis = r#"{"db": {"entries": [{"name": "GitHub", "issuer": "GitHub", "info": {"secret": "JBSWY3DPEHPK3PXP"}}]}}"#;
    let aegis_items = parse_aegis_json(sample_aegis);
    assert_eq!(aegis_items.len(), 1);
    assert!(aegis_items[0].title.contains("GitHub"));
    let aegis_exp = export_aegis_json(&aegis_items);
    assert!(aegis_exp.contains("GitHub"));

    let sample_2fas = r#"{"services": [{"name": "AWS Root 2FA", "secret": "JBSWY3DPEHPK3PXP"}]}"#;
    let twofas_items = parse_2fas_json(sample_2fas);
    assert_eq!(twofas_items.len(), 1);
    assert_eq!(twofas_items[0].title, "AWS Root 2FA");
    let twofas_exp = export_2fas_json(&twofas_items);
    assert!(twofas_exp.contains("AWS Root 2FA"));
}

#[test]
fn test_azure_and_terraform() {
    let kv_json = r#"{"DB_CONNECTION_STRING": "Server=tcp:prod.database.windows.net"}"#;
    let kv_items = parse_azure_kv_json(kv_json);
    assert_eq!(kv_items.len(), 1);
    assert_eq!(kv_items[0].title, "DB_CONNECTION_STRING");
    let kv_exp = export_azure_kv_json(&kv_items);
    assert!(kv_exp.contains("DB_CONNECTION_STRING"));

    let tf_json = r#"{"api_gateway_secret": "tfsecret123"}"#;
    let tf_items = parse_tfvars_json(tf_json);
    assert_eq!(tf_items.len(), 1);
    assert_eq!(tf_items[0].title, "api_gateway_secret");
    let tf_exp = export_tfvars_json(&tf_items);
    assert!(tf_exp.contains("api_gateway_secret"));
}

#[test]
fn test_all_other_exporters() {
    let items = vec![VaultItem::new(ItemType::Login, "EnterpriseApp", ItemData::Login(LoginData {
        username: Some("admin".into()),
        password: Some("p@ssword".into()),
        urls: vec!["https://app.corp".into()]
    }))];

    assert!(export_safeincloud_xml(&items).contains("EnterpriseApp"));
    assert!(export_buttercup_json(&items).contains("EnterpriseApp"));
    assert!(export_passbolt_csv(&items).contains("EnterpriseApp"));
    assert!(export_roboform_csv(&items).contains("EnterpriseApp"));
}
