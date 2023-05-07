use std::{collections::HashMap, fmt::format};
use futures_util::StreamExt;

use super::file_utils;
use crate::config;
const DOC_SERV_FILLFORMS : [&str;2] = [".oform", ".docx"];
const DOC_SERV_VIEWED : [&str;4] = [".pdf", ".djvu", ".xps", ".oxps"] ; //# file extensions that can be viewed
const DOC_SERV_EDITED  : [&str;6]= [".docx", ".xlsx", ".csv", ".pptx", ".txt", ".docxf"];  //# file extensions that can be edited
const DOC_SERV_CONVERT : [&str;35] = [                                           //# file extensions that can be converted
    ".docm", ".doc", ".dotx", ".dotm", ".dot", ".odt",
    ".fodt", ".ott", ".xlsm", ".xlsb", ".xls", ".xltx", ".xltm",
    ".xlt", ".ods", ".fods", ".ots", ".pptm", ".ppt",
    ".ppsx", ".ppsm", ".pps", ".potx", ".potm", ".pot",
    ".odp", ".fodp", ".otp", ".rtf", ".mht", ".html", ".htm", ".xml", ".epub", ".fb2"
];
const DOCUMENT_SERVER_URL : &str = "http://localhost:8000/";
const ROOT_FOLDER : &str = "app_data/";
pub struct DocumentManager;

impl DocumentManager{
    pub fn new() -> Self{
        DocumentManager{}
    }
    pub fn is_can_fill_forms(&self,file_extension : &str) -> bool{
        DOC_SERV_FILLFORMS.contains(&file_extension)
    }

    pub fn is_can_view(&self,file_extension : &str)-> bool{
        DOC_SERV_VIEWED.contains(&file_extension)
    }

    pub fn is_can_edit (&self,file_extension : &str) -> bool{
        DOC_SERV_EDITED.contains(&file_extension)
    }

    pub fn is_can_convert(&self,file_extension : &str) -> bool{
        DOC_SERV_CONVERT.contains(&file_extension)
    }

    pub fn is_supported_extension(&self,file_extension : &str) -> bool{
        self.is_can_fill_forms(file_extension) || self.is_can_view(file_extension) || self.is_can_edit(file_extension) || self.is_can_convert(file_extension)
    }

    pub fn get_internal_extension(&self,file_type : &str) -> String{
        let mapping = HashMap::from([
            ("word",".docx"),
            ("cell",".xlsx"),
            ("slide",".pptx"),
            ("docxf",".docxf")
        ]);

        mapping.get(file_type).map_or(".docx".to_owned(), |f| f.to_string())
    }

    pub fn get_template_image_url(&self,filetype : &str) -> String{
        let path = format!("{}{}",self.get_server_url(),"/static/images/");
        let mapping = HashMap::from([
            ("word", format!("{}{}",path,"file_docx.svg")),
            ("cell", format!("{}{}",path,"file_xlsx.svg")),
            ("slide",format!("{}{}",path,"file_pptx.svg")),
        ]);
        mapping.get(filetype).map_or("file_docx.svg".to_string(), |f| f.to_string())
    }

    pub fn get_correct_name(&self,filename : &str) -> String{
        let basename = file_utils::get_file_name_no_ext(filename);
        let ext = file_utils::get_file_ext(filename);

        let mut name = format!("{basename}{ext}");
        let mut i = 1;

        while std::path::Path::new(&name).exists() {
            name = format!("{basename}({i}){ext}");
            i +=1;
        }
        name
    }

    pub fn get_server_url(&self) -> String{
        DOCUMENT_SERVER_URL.to_owned()
    }

    pub fn get_file_uri(&self,filename : &str)-> String{
        let host = self.get_server_url();
        format!("{}{}/{}",host,config::STATIC_URL,filename)
    }


    pub async fn get_storage_path(&self,filename : &str,user_id : i32) -> String{
        let user_folder = format!("{}{}",ROOT_FOLDER,user_id);
        log::debug!("Folder {}", user_folder);
        match std::path::Path::new(&user_folder).exists() {
            true => (),
            false => tokio::fs::create_dir(&user_folder).await.unwrap(),
        }
        let destination = format!(
            "{}/{}",
            user_folder,
            filename
        );
        destination
    }
    pub async fn create_file(&self,stream : &mut tokio::fs::File,path : &str, meta : bool) {
        use tokio::fs::File;
        use tokio::io::{AsyncWriteExt,AsyncReadExt};
        let mut buf = bytes::BytesMut::with_capacity(8192);
        let mut read = stream.read_buf(&mut buf).await.unwrap();
        let mut out = File::create(path).await.unwrap();
        while read > 0 {
            out.write(&buf).await.unwrap();
            read = stream.read_buf(&mut buf).await.unwrap();
        }
    }

    // TODO IMPLEMENT PROPER PATH CHECKS!!!!
    pub async fn create_sample(&self,filetype : &str, is_sample : bool, user_id : i32) -> String{
        let ext = self.get_internal_extension(filetype);
        let mut sample_name = String::from("new");
        if is_sample{
            sample_name = "sample".to_string();
        }
        let filename = self.get_correct_name(&format!("{sample_name}{ext}"));
        let path = self.get_storage_path(&filename,user_id).await;
        let mut asset_file = tokio::fs::File::open(
            format!("assets/{}/{}",sample_name,filename)
        ).await.unwrap();
        self.create_file(&mut asset_file, &path, true).await;
        return filename;
    }
}