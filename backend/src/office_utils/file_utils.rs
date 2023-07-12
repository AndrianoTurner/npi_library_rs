#![allow(non_snake_case,unused,dead_code)]
use std::ffi::OsStr;

use crate::config;
use crate::error::Error;
use std::path::Path;
type Result<T> = std::result::Result<T,Error>;
/// Возвращает имя файла с расширением
/// 
/// # Пример:
/// ```
///     let filename = "app_data/10/aboba.docx";
///     assert_eq!(get_file_name(filename).unwrap(),"aboba.docx".to_string());
/// ```
pub fn get_file_name(filename : &Path) -> Result<String>{
    let res = filename
        .file_name()
        .unwrap_or(OsStr::new(""))
        .to_string_lossy().
        to_string();
    if res.is_empty(){return Err(Error::FileUtils);}
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
pub fn get_file_name_no_ext(filename : &Path) -> Result<String>{
    let res = filename
        .file_stem()
        .unwrap_or(OsStr::new(""))
        .to_string_lossy().
        to_string();
    if res.is_empty(){return Err(Error::FileUtils);}
    Ok(res)
}
/// Возвращает расширение файла
/// 
/// # Пример:
/// ```
///     let filename = "app_data/10/aboba.docx";
///     assert_eq!(get_file_name(filename).unwrap(),"aboba.docx".to_string());
/// ```
pub fn get_file_ext(filename : &Path) -> Result<String>{
    let res = filename
        .extension()
        .unwrap_or(OsStr::new(""))
        .to_string_lossy().
        to_string();
    if res.is_empty() {return Err(Error::FileUtils);}
    Ok(res)
}

pub fn get_file_type(filename : &Path) -> Result<String>{
    let ext = get_file_ext(filename)?;

    if config::EXT_DOCUMENT.contains(&ext.as_str()){Ok("word".to_string())}
    else if config::EXT_PRESENTATION.contains(&ext.as_str()){Ok("slide".to_string())}
    else if config::EXT_SPREADSHEET.contains(&ext.as_str()) {Ok("cell".to_string())}
    else{
        Err(Error::FileUtils)
    }
}

#[cfg(test)]

mod tests{
    use super::*;
    #[test]
    fn test_get_file_name(){
        let filename = Path::new("app_data/10/aboba.docx");
        let filename2 = Path::new("app_data/10/aboba1.docx/");
        let wrong2 = Path::new("///");
        let name = Path::new("/aboba.docx");
        assert_eq!(get_file_name(filename).unwrap(),"aboba.docx".to_owned());
        assert_eq!(get_file_name(filename2).unwrap(),"aboba1.docx".to_owned());
        assert_eq!(get_file_name(wrong2),Err(Error::FileUtils));
        assert_eq!(get_file_name(name).unwrap(),"aboba.docx".to_string())
    }

    fn test_get_file_no_ext(){
        let a = Path::new("app_data/10/aboba.docx");
        let b = Path::new("aboba.docx.rtf");
        let c = Path::new("a.");
        assert_eq!(get_file_name_no_ext(a).unwrap(),"aboba".to_string());
        assert_eq!(get_file_name_no_ext(b).unwrap(),"aboba.docx".to_string());
        assert_eq!(get_file_name_no_ext(c),Err(Error::FileUtils));
    }

    fn test_get_file_ext(){
        let a = Path::new("app_data/10/aboba.docx");
        let b = Path::new("aboba.docx.rtf");
        let c = Path::new("a.");

        assert_eq!(get_file_ext(a).unwrap(),"docx".to_string());
        assert_eq!(get_file_name_no_ext(b).unwrap(),"rtf".to_string());
        assert_eq!(get_file_name_no_ext(c),Err(Error::FileUtils));
    }

    fn test_file_type(){
        let a = Path::new("docx");
        let b = Path::new("xlsx");
        let c = Path::new("pptx");

        assert_eq!(get_file_type(a).unwrap(),"word".to_owned());
        assert_eq!(get_file_type(b).unwrap(),"cell".to_owned());
        assert_eq!(get_file_type(c).unwrap(),"slide".to_owned());
    }
}