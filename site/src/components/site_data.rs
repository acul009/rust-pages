use std::path::Path;

pub const NAME: &str = "Rahn IT-Systemtechnik";
pub const STREET: &str = "Alte Bahn 9";
pub const POSTAL_CODE: &str = "84577";
pub const CITY: &str = "TÃ¼ÃŸling";
pub const PHONE: &str = "08633 / 977969 - 0";
pub const MAIL: &str = "info@it-rahn.de";
pub const UST_ID: &str = "DE206337243";
pub const IBAN: &str = "DE95 7601 0085 0059 3418 53";

pub fn asset_path(relative: &str) -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(relative)
        .to_string_lossy()
        .into_owned()
}

pub fn tel_href() -> String {
    format!("tel:+49{}", &PHONE[1..].replace([' ', '/'], ""))
}
