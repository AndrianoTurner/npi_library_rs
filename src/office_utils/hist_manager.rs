
#![allow(non_snake_case,unused,dead_code)]

use crate::database;
pub fn get_history_dir(storage_path : &str) -> String{
    format!("{}-hist",storage_path)
}

pub fn get_version_dir(hist_dir : &str , version : i32) -> String{
    format!("{}/{}",hist_dir,version)
}

pub fn get_file_version(hist_dir : &str) -> i32{
    let dir = std::path::Path::new(hist_dir);
    if !dir.exists(){
        return 0
    }

    let mut count = 1;

    for entry in dir.read_dir().unwrap().flatten(){
            if entry.path().is_dir(){
                count += 1; 
            }   
    }
    count
}

pub async fn get_next_version_dir(hist_dir : &str) -> String{
    let v = get_file_version(hist_dir);
    let p = get_version_dir(hist_dir,v);
    let path = std::path::Path::new(&p);

    if !path.exists(){
        tokio::fs::create_dir(path).await.unwrap();
    }

    path.to_string_lossy().to_string()
}

pub fn get_changes_zip_path(ver_dir : &str) -> String{
    format!("{}/diff.zip",ver_dir)
}
pub fn get_changes_history_path(ver_dir : &str) -> String{
    format!("{}/changes.json",ver_dir)
}

pub fn get_prev_file_path(ver_dir : &str, ext : &str) -> String{
    format!("{}/prev{}",ver_dir,ext)
}

pub fn get_key_path(ver_dir : &str) -> String{
    format!("{}/key.txt",ver_dir)
}

pub fn get_meta_path(hist_dir : &str) -> String{
    format!("{}/createdInfo.json",hist_dir)
}

pub async fn create_meta(storage_path : &str, user_id : i32){
    let hist_dir = get_history_dir(storage_path);
    let path = get_meta_path(&hist_dir);

    let p = std::path::Path::new(&path);

    if !p.exists(){
        tokio::fs::create_dir(p).await.unwrap()
    }
}