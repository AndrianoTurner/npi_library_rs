
#![allow(non_snake_case,unused,dead_code)]
use std::path::{Path,PathBuf};
use chrono::prelude::*;
use tokio::io::AsyncWriteExt;
use crate::database;
use super::doc_manager;


#[derive(serde::Serialize,serde::Deserialize)]
    pub struct MetaData{
        pub created : DateTime<Local>,
        pub uid : i32,
        pub uname : String,
    }


pub fn get_history_dir(storage_path : &Path) -> PathBuf{
    PathBuf::from(format!("{}-hist",storage_path.display()))
}

pub fn get_version_dir(hist_dir : &Path , version : i32) -> PathBuf{
    PathBuf::from(format!("{}/{}",hist_dir.display(),version))
}

pub fn get_file_version(hist_dir : &Path) -> i32{
    if !hist_dir.exists(){
        return 0
    }

    let mut count = 1;

    for entry in hist_dir.read_dir().unwrap().flatten(){
            if entry.path().is_dir(){
                count += 1; 
            }   
    }
    count
}

pub async fn get_next_version_dir(hist_dir : &Path) -> PathBuf{
    let v = get_file_version(hist_dir);
    let path = get_version_dir(hist_dir,v);

    if !path.exists(){
        tokio::fs::create_dir(&path).await.unwrap();
    }

    path
}

pub fn get_changes_zip_path(ver_dir : &Path) -> PathBuf{
    PathBuf::from(format!("{}/diff.zip",ver_dir.display()))
    
}
pub fn get_changes_history_path(ver_dir : &Path) -> PathBuf{
    PathBuf::from(format!("{}/changes.json",ver_dir.display()))
}

pub fn get_prev_file_path(ver_dir : &Path, ext : &str) -> PathBuf{
    PathBuf::from(format!("{}/prev.{}",ver_dir.display(),ext))
}

pub fn get_key_path(ver_dir : &Path) -> PathBuf{
    PathBuf::from(format!("{}/key.txt",ver_dir.display()))
}

pub fn get_meta_path(hist_dir : &Path) -> PathBuf{
    PathBuf::from(format!("{}/createdInfo.json",hist_dir.display()))
}


pub async fn create_metadata(storage_path : &Path, user_id : i32,username : &str) -> Result<(),tokio::io::Error>{
    let hist_dir = get_history_dir(storage_path);
    let path = get_meta_path(&hist_dir);

    if !hist_dir.exists(){
        tokio::fs::create_dir(&hist_dir).await?
    }
    

    let metadata = MetaData{
        created : Local::now(),
        uid : user_id,
        uname : username.to_owned()
    };

    let mut file = tokio::fs::File::create(&path).await?;
    let json = serde_json::to_string(&metadata).unwrap();
    file.write_all(json.as_bytes()).await?;
    Ok(())
}

pub fn get_public_history_uri(
    filename : &str,
    version : i32,
    file : &str,
    user_id : i32) -> String {

        let host = doc_manager::get_server_url();
        format!("{host}/downloadhistory?filename={filename}&ver={version}&file={file}{user_id}")
}

pub async fn get_meta(storage_path : &Path) -> Option<MetaData>{
    let hist_dir = get_history_dir(storage_path);
    let path = get_meta_path(&hist_dir);

    if !path.exists() {return None;}

    let string = tokio::fs::read_to_string(&path).await.unwrap();

    Some(serde_json::from_str::<MetaData>(&string).unwrap())
}