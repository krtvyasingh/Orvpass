use crate::models::VaultItem;

pub fn parse_vivaldi_csv(csv_str: &str) -> Vec<VaultItem> {
    crate::import_export::parsers::chrome::parse_chrome_csv(csv_str)
}
