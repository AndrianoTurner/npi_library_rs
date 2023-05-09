use crate::office_utils::doc_manager;

use super::models::{CallbackData};
use super::file_utils::{self, PathParseError};
#[derive(Debug)]
pub struct TrackManagerError;
impl std::fmt::Display for TrackManagerError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f," track manager error!")
    }
}
impl std::error::Error for TrackManagerError{}

impl From<PathParseError> for TrackManagerError{
    fn from(value: PathParseError) -> Self {
        Self {}
    }
}

pub async fn process_save(body : CallbackData,filename : &str) -> Result<(),TrackManagerError>{
    use super::service_converter;
    let mut download = body.url.ok_or(TrackManagerError)?;
    let changesuri = body.changesurl.ok_or(TrackManagerError)?;
    let mut new_file_name = filename.to_string();
    let cur_ext = file_utils::get_file_ext(filename)?;
    let filetype = body.filetype.ok_or(TrackManagerError)?;
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
                let f = format!("{}{}",file_utils::get_file_name_no_ext(filename)?,download_ext);
                new_file_name = doc_manager::get_correct_name(&f)?
            } ,
        }
    }
    todo!();
    //let path = doc_manager::get_storage_path(&new_file_name, user_id).await;
    
    Ok(())
}