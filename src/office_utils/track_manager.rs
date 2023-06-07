#![allow(non_snake_case,unused,dead_code)]
use std::path::PathBuf;

use crate::config;
use crate::office_utils::{doc_manager, hist_manager};

use super::models::{CallbackData};
use super::file_utils::{self};
use super::service_converter;
use crate::error::Error;

type Result<T> = std::result::Result<T,Error>;

pub async fn process_save(body : &CallbackData,filename : &str, user_id : i32) -> Result<()>{
    use super::service_converter;
    let body = body.clone();
    let mut download = body.url.ok_or(Error::Track)?;
    let changesuri = body.changesurl.ok_or(Error::Track)?;
    let mut new_file_name = filename.to_string();
    let cur_ext = file_utils::get_file_ext(std::path::Path::new(filename))?;
    let filetype = body.filetype.ok_or(Error::Track)?;
    let download_ext = format!(".{}",filetype);

    if cur_ext != download_ext{
        let new_uri = service_converter::get_converter_uri(
            &download,
            &download_ext,
            &cur_ext,
            &doc_manager::generate_revision_id(&download)
        ).await;

        match new_uri {
            Ok(u) => {
               download = u
            }
            Err(e) => {
                let f = format!("{}{}",file_utils::get_file_name_no_ext(std::path::Path::new(filename))?,download_ext);
                new_file_name = doc_manager::get_correct_name(&f)?
            } ,
        }
    }
    let path = doc_manager::get_storage_path(&new_file_name, user_id).await;
    let hist_dir = hist_manager::get_history_dir(&path);
    if !hist_dir.exists(){
        tokio::fs::create_dir(&hist_dir).await.unwrap();
    }
    let version_dir = hist_manager::get_next_version_dir(&hist_dir).await;
    let prev = hist_manager::get_prev_file_path(&version_dir, &cur_ext);
    tokio::fs::rename(&path,&prev).await;

    doc_manager::save_file_from_uri(&download, &path);
    let changess_zip_path = hist_manager::get_changes_zip_path(&version_dir);
    doc_manager::save_file_from_uri(&changesuri, &changess_zip_path);
    Ok(())
}

pub async fn process_force_save(body : CallbackData, filename : &str, user_id : i32) -> Result<()> {
    if let None = body.url {
        return Err(Error::Track);
    }
    if let None = body.filetype{
        return Err(Error::Track);
    }
    let filetype = body.filetype.unwrap();
    let mut filename = String::from(filename);
    let mut download = body.url.unwrap();
    let mut forcesave_path = PathBuf::new();
    let cur_ext = file_utils::get_file_ext(std::path::Path::new(&filename))?;
    let mut new_filename = false;

    if (cur_ext != filetype ){
        let new_uri = service_converter::get_converter_uri(
            &download, 
            &filetype, 
            &cur_ext, 
            &doc_manager::generate_revision_id(&download)).await;
        match new_uri{
            Ok(uri) => download = uri,
            Err(_) => new_filename = true,
        }
    }
    let is_submit_form = body.forcesavetype.unwrap() == 3;

    if is_submit_form{
        if new_filename{
            filename = doc_manager::get_correct_name(&format!("{}-form{}",file_utils::get_file_name_no_ext(&std::path::Path::new(&filename))?,filetype))?
        }
        else{
            filename = doc_manager::get_correct_name(&format!("{}-form{}",file_utils::get_file_name_no_ext(&std::path::Path::new(&filename))?,cur_ext))?
        }
        forcesave_path = doc_manager::get_forcesave_path(&filename, user_id, true).await.unwrap();
    }
    else{
        if new_filename{
            filename = doc_manager::get_correct_name(&format!("{}{}",file_utils::get_file_name_no_ext(&std::path::Path::new(&filename))?,filetype))?
        }
        forcesave_path = doc_manager::get_forcesave_path(&filename, user_id, false).await.unwrap();
        if forcesave_path.to_str().unwrap().len() == 0{
            forcesave_path = doc_manager::get_forcesave_path(&filename, user_id, true).await.unwrap();
        }
    }

    doc_manager::save_file_from_uri(&download, &forcesave_path);

    Ok(())
}

pub async fn command_request(method : &str, key : &str, meta : Option<String>){
    let command_url = format!("{}{}",config::DOCUMENT_SERVER_URL,config::DOC_SERV_COMMAND_URL);
    #[derive(serde::Serialize)]
    struct Payload{
        c : String,
        key  : String,
        meta : Option<String>,
    }
    let payload = Payload {c : method.to_string(),key : key.to_string(),meta};
    let client = reqwest::Client::new();
    let response = client.post(command_url)
        .json(&payload)
        .send()
        .await.unwrap();

}