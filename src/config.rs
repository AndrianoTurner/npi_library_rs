use std::collections::HashMap;

use once_cell::sync::Lazy;

const EXT_SPREADSHEET : [&str;11] = [
    ".xls", ".xlsx", ".xlsm", ".xlsb",
    ".xlt", ".xltx", ".xltm",
    ".ods", ".fods", ".ots", ".csv"
];

const EXT_PRESENTATION : [&str;12] = [
    ".pps", ".ppsx", ".ppsm",
    ".ppt", ".pptx", ".pptm",
    ".pot", ".potx", ".potm",
    ".odp", ".fodp", ".otp"
];

const EXT_DOCUMENT : [&str;22] = [
    ".doc", ".docx", ".docm",
    ".dot", ".dotx", ".dotm",
    ".odt", ".fodt", ".ott", ".rtf", ".txt",
    ".html", ".htm", ".mht", ".xml",
    ".pdf", ".djvu", ".fb2", ".epub", ".xps", ".oxps", ".oform"
];

static LANGUAGES: Lazy<HashMap<&str,&str>> = Lazy::new(|| HashMap::from(
    [
        ("en", "English"),
        ("ru", "Русский"),
    ]));