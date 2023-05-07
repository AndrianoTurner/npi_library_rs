use std::collections::HashMap;

use once_cell::sync::Lazy;
pub const STATIC_URL : &str = "/static";
pub const API_ROUTE : &str = "/office";
pub const EXT_SPREADSHEET : [&str;11] = [
    ".xls", ".xlsx", ".xlsm", ".xlsb",
    ".xlt", ".xltx", ".xltm",
    ".ods", ".fods", ".ots", ".csv"
];

pub const EXT_PRESENTATION : [&str;12] = [
    ".pps", ".ppsx", ".ppsm",
    ".ppt", ".pptx", ".pptm",
    ".pot", ".potx", ".potm",
    ".odp", ".fodp", ".otp"
];

pub const EXT_DOCUMENT : [&str;22] = [
    ".doc", ".docx", ".docm",
    ".dot", ".dotx", ".dotm",
    ".odt", ".fodt", ".ott", ".rtf", ".txt",
    ".html", ".htm", ".mht", ".xml",
    ".pdf", ".djvu", ".fb2", ".epub", ".xps", ".oxps", ".oform"
];

pub static LANGUAGES: Lazy<HashMap<&str,&str>> = Lazy::new(|| HashMap::from(
    [
        ("en", "English"),
        ("ru", "Русский"),
    ]));