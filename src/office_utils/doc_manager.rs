use std::{collections::HashMap, fmt::format};
use futures_util::StreamExt;
use tokio::io::{AsyncRead, AsyncReadExt};
use std::path::Path;
use super::file_utils::{self, PathParseError};
use crate::config::{
    EXT_DOCUMENT,
    EXT_PRESENTATION,
    EXT_SPREADSHEET,
    DOC_SERV_VIEWED,
    DOC_SERV_EDITED,
    DOC_SERV_CONVERT,
    DOC_SERV_FILLFORMS,
    DOCUMENT_SERVER_URL,
    STATIC_URL,
    ROOT_FOLDER,
    DOC_SERV_API_URL
};

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
            ("word","docx"),
            ("cell","xlsx"),
            ("slide","pptx"),
            ("docxf","docxf")
        ]);

        mapping.get(file_type).map_or("docx".to_owned(), |f| f.to_string())
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

    pub fn get_correct_name(&self,filename : &str) -> Result<String,PathParseError>{ 
        let basename = file_utils::get_file_name_no_ext(filename)?;
        let ext = file_utils::get_file_ext(filename)?;

        let mut name = format!("{basename}{ext}");
        let mut i = 1;

        while std::path::Path::new(&name).exists() {
            name = format!("{basename}({i}){ext}");
            i +=1;
        }
        Ok(name)
    }

    pub fn get_server_url(&self) -> String{
        DOCUMENT_SERVER_URL.to_owned()
    }

    pub fn get_file_uri(&self,filename : &str, user_id : i32)-> String{
        let host = self.get_server_url();
        format!("{}{}/{}/{}",host,STATIC_URL,user_id,filename)
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

    pub async fn create_file<T>(&self,stream : &mut T,path : &str, meta : bool)
        where 
        T : std::marker::Unpin  + AsyncRead + AsyncReadExt
    {
        use tokio::fs::File;
        use tokio::io::{AsyncWriteExt,AsyncReadExt};
        let mut out = File::create(path).await.unwrap();
        let mut buf  = [0u8;8192];
        while let Ok(b) = stream.read(&mut buf).await {
            out.write_all(&buf);
        }
    }

    // TODO IMPLEMENT PROPER PATH CHECKS!!!!
    pub async fn create_sample(&self,filetype : &str, is_sample : bool, user_id : i32) -> Result<String,PathParseError>{
        let ext = self.get_internal_extension(filetype);
        let mut sample_name = String::from("new");
        if is_sample{
            sample_name = "sample".to_string();
        }
        let filename = self.get_correct_name(&format!("{sample_name}{ext}"))?;
        let path = self.get_storage_path(&filename,user_id).await;
        let mut asset_file = tokio::fs::File::open(
            format!("assets/{}/{}",sample_name,filename)
        ).await.unwrap();
        self.create_file(&mut asset_file, &path, true).await;
        Ok(filename)
    }

    pub async fn remove_file(&self,filename : &str, user_id : i32){
        let path = self.get_storage_path(filename, user_id).await;
        let path = std::path::Path::new(&path);
        if path.exists(){
            // По идее должно быть безопасно, т.к путь точно существует
            // хотя может зафейлить во время конкретно удаления
            tokio::fs::remove_file(path).await.unwrap();
        }
        
    }

    pub async fn generate_file_key(&self,filename : &str, user_id : i32) -> String{
        let path = self.get_storage_path(filename, user_id).await;
        let uri = self.get_file_uri(filename,user_id);
        let file = tokio::fs::File::open(path).await.unwrap();
        let metadata = file.metadata().await.unwrap();
        let last_mofif : u64 = metadata.modified().unwrap().duration_since(metadata.created().unwrap()).unwrap().as_secs();
        let input = format!("{}_{}",uri,last_mofif);
        let hash = xxhash_rust::xxh64::xxh64(input.as_bytes(), 1).to_string();
        hash[..19].to_string()
    }

    pub async fn get_js_scripts(&self) -> Result<String,reqwest::Error>{
        use reqwest;
        let script = reqwest::get(format!("{}{}",self.get_server_url(),DOC_SERV_API_URL)).await?;
        script.text().await
    }

    pub async fn get_file_for_user(&self,filename : &str, user_id : i32) -> Result<Vec<u8>,PathParseError>{
        let storage = &self.get_storage_path(filename, user_id).await;
        let file = Path::new(&storage);

        if !file.exists(){return Err(PathParseError)}

        Ok(tokio::fs::read(file).await.map_err(|_| PathParseError).unwrap())
    }

}

#[cfg(test)]

mod tests{
    use super::DocumentManager;

    #[test]
    fn test_is_can(){
        let dm = DocumentManager::new();
        let good_convert = "docm";
        let good = "docx";
        let wrong = "idk";
        assert_eq!(dm.is_can_convert(good_convert),true);
        assert_eq!(dm.is_can_convert(wrong),false);
        assert_eq!(dm.is_can_edit(good),true);
        assert_eq!(dm.is_can_edit(wrong),false);
        assert_eq!(dm.is_can_fill_forms(good),true);
        assert_eq!(dm.is_can_fill_forms(wrong),false);
        assert_eq!(dm.is_can_view("pdf"),true);
        assert_eq!(dm.is_can_view(wrong),false);
        assert_eq!(dm.is_supported_extension(good),true);
        assert_eq!(dm.is_supported_extension(wrong),false);
    }

    #[tokio::test]
    async fn test_generate_file_key() {
        let manager = DocumentManager::new();
        let key1 = manager.generate_file_key("test.txt", 8).await;
        let key2 = manager.generate_file_key("test.txt", 8).await;
        assert_eq!(key1,key2);
    }
    #[tokio::test]
    async fn test_get_js(){
        let manager = DocumentManager::new();
        let js = manager.get_js_scripts().await.unwrap();  
        println!("{}",js);
    }
    #[tokio::test]
    async fn test_remove_file(){
        let documentmanager = DocumentManager::new();
        documentmanager.remove_file("test.txt", 8).await;
    }
}