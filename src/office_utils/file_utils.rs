#![allow(non_snake_case,unused,dead_code)]
use std::ffi::OsStr;

use crate::config;

#[derive(Debug,PartialEq)]
pub struct PathParseError;

impl std::error::Error for PathParseError{}

impl std::fmt::Display for PathParseError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f," couldn't parse the filename")
    }
}
/// Возвращает имя файла с расширением
/// 
/// # Пример:
/// ```
///     let filename = "app_data/10/aboba.docx";
///     assert_eq!(get_file_name(filename).unwrap(),"aboba.docx".to_string());
/// ```
pub fn get_file_name(filename : &str) -> Result<String,PathParseError>{
    let res = std::path::Path::new(filename)
        .file_name()
        .unwrap_or(OsStr::new(""))
        .to_string_lossy().
        to_string();
    if res.is_empty(){return Err(PathParseError);}
    Ok(res)
}
// Функции были взяты из питоновского примера, их надо проверить

/// Возвращает имя файла без расширения
/// 
/// # Пример:
/// ```
///     let filename = "app_data/10/aboba.docx";
///     assert_eq!(get_file_name(filename).unwrap(),"aboba".to_string());
/// ```
pub fn get_file_name_no_ext(filename : &str) -> Result<String,PathParseError>{
    let res = std::path::Path::new(filename)
        .file_stem()
        .unwrap_or(OsStr::new(""))
        .to_string_lossy().
        to_string();
    if res.is_empty(){return Err(PathParseError);}
    Ok(res)
}
/// Возвращает расширение файла
/// 
/// # Пример:
/// ```
///     let filename = "app_data/10/aboba.docx";
///     assert_eq!(get_file_name(filename).unwrap(),"aboba.docx".to_string());
/// ```
pub fn get_file_ext(filename : &str) -> Result<String,PathParseError>{
    let res = std::path::Path::new(filename)
        .extension()
        .unwrap_or(OsStr::new(""))
        .to_string_lossy().
        to_string();
    if res.is_empty() {return Err(PathParseError)};
    Ok(res)
}

pub fn get_file_type(filename : &str) -> Result<String,PathParseError>{
    let ext = get_file_ext(filename)?;

    if config::EXT_DOCUMENT.contains(&ext.as_str()){Ok("word".to_string())}
    else if config::EXT_PRESENTATION.contains(&ext.as_str()){Ok("slide".to_string())}
    else if config::EXT_SPREADSHEET.contains(&ext.as_str()) {Ok("cell".to_string())}
    else{
        Err(PathParseError)
    }
}

#[cfg(test)]

mod tests{
    use super::*;
    #[test]
    fn test_get_file_name(){
        let filename = "app_data/10/aboba.docx";
        let filename2 = "app_data/10/aboba1.docx/";
        let wrong2 = "///";
        let name = "/aboba.docx";
        assert_eq!(get_file_name(filename).unwrap(),"aboba.docx".to_owned());
        assert_eq!(get_file_name(filename2).unwrap(),"aboba1.docx".to_owned());
        assert_eq!(get_file_name(wrong2),Err(PathParseError));
        assert_eq!(get_file_name(name).unwrap(),"aboba.docx".to_string())
    }

    fn test_get_file_no_ext(){
        let a = "app_data/10/aboba.docx";
        let b = "aboba.docx.rtf";
        let c = "a.";
        assert_eq!(get_file_name_no_ext(a).unwrap(),"aboba".to_string());
        assert_eq!(get_file_name_no_ext(b).unwrap(),"aboba.docx".to_string());
        assert_eq!(get_file_name_no_ext(c),Err(PathParseError));
    }

    fn test_get_file_ext(){
        let a = "app_data/10/aboba.docx";
        let b = "aboba.docx.rtf";
        let c = "a.";

        assert_eq!(get_file_ext(a).unwrap(),"docx".to_string());
        assert_eq!(get_file_name_no_ext(b).unwrap(),"rtf".to_string());
        assert_eq!(get_file_name_no_ext(c),Err(PathParseError));
    }

    fn test_file_type(){
        let a = "docx";
        let b = "xlsx";
        let c = "pptx";

        assert_eq!(get_file_type(a).unwrap(),"word".to_owned());
        assert_eq!(get_file_type(b).unwrap(),"cell".to_owned());
        assert_eq!(get_file_type(c).unwrap(),"slide".to_owned());
    }
}