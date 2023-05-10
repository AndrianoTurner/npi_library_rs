#![allow(non_snake_case,unused)]
use std::collections::HashMap;

use once_cell::sync::Lazy;
pub const STATIC_URL : &str = "/static";
pub const API_ROUTE : &str = "/office";
pub const EXT_SPREADSHEET : [&str;11] = [
    "xls", "xlsx", "xlsm", "xlsb",
    "xlt", "xltx", "xltm",
    "ods", "fods", "ots", "csv"
];

pub const EXT_PRESENTATION : [&str;12] = [
    "pps", "ppsx", "ppsm",
    "ppt", "pptx", "pptm",
    "pot", "potx", "potm",
    "odp", "fodp", "otp"
];

pub const EXT_DOCUMENT : [&str;22] = [
    "doc", "docx", "docm",
    "dot", "dotx", "dotm",
    "odt", "fodt", "ott", "rtf", "txt",
    "html", "htm", "mht", "xml",
    "pdf", "djvu", "fb2", "epub", "xps", "oxps", "oform"
];


pub const DOC_SERV_FILLFORMS : [&str;2] = ["oform", "docx"];
pub const DOC_SERV_VIEWED : [&str;4] = ["pdf", "djvu", "xps", "oxps"] ; //# file extensions that can be viewed
pub const DOC_SERV_EDITED  : [&str;6]= ["docx", "xlsx", "csv", "pptx", "txt", "docxf"];  //# file extensions that can be edited
pub const DOC_SERV_CONVERT : [&str;35] = [                                           //# file extensions that can be converted
    "docm", "doc", "dotx", "dotm", "dot", "odt",
    "fodt", "ott", "xlsm", "xlsb", "xls", "xltx", "xltm",
    "xlt", "ods", "fods", "ots", "pptm", "ppt",
    "ppsx", "ppsm", "pps", "potx", "potm", "pot",
    "odp", "fodp", "otp", "rtf", "mht", "html", "htm", "xml", "epub", "fb2"
];
pub const DOCUMENT_SERVER_URL : &str = "http://localhost:8000/";
pub const DOC_SERV_CONVERTER_URL : &str = "ConvertService.ashx";
pub const DOC_SERV_API_URL : &str = "web-apps/apps/api/documents/api.js";
pub const DOC_SERV_PRELOADER_URL : &str = "web-apps/apps/api/documents/cache-scripts.html";
pub const DOC_SERV_COMMAND_URL : &str = "coauthoring/CommandService.ashx";
pub const ROOT_FOLDER : &str = "app_data/";

pub static LANGUAGES: Lazy<HashMap<&str,&str>> = Lazy::new(|| HashMap::from(
    [
        ("en", "English"),
        ("ru", "Русский"),
    ]));