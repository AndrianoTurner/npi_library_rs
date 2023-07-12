#![allow(non_snake_case,unused)]
use std::{collections::HashMap};
use actix_web::http::header::{HeaderMap, self,HeaderValue};
use actix_web::{HttpResponse, HttpResponseBuilder};
use reqwest::StatusCode;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};
use std::path::Path;
use std::path::PathBuf;
use super::file_utils::{self};
use crate::config::{
    DOC_SERV_VIEWED,
    DOC_SERV_EDITED,
    DOC_SERV_CONVERT,
    DOC_SERV_FILLFORMS,
    DOCUMENT_SERVER_URL,
    STATIC_URL,
    ROOT_FOLDER,
    DOC_SERV_API_URL
};
use crate::error::Error;
    type Result<T> = std::result::Result<T,Error>;
    pub fn is_can_fill_forms(file_extension : &str) -> bool{
        DOC_SERV_FILLFORMS.contains(&file_extension)
    }

    pub fn is_can_view(file_extension : &str)-> bool{
        DOC_SERV_VIEWED.contains(&file_extension)
    }

    pub fn is_can_edit (file_extension : &str) -> bool{
        DOC_SERV_EDITED.contains(&file_extension)
    }

    pub fn is_can_convert(file_extension : &str) -> bool{
        DOC_SERV_CONVERT.contains(&file_extension)
    }

    pub fn is_supported_extension(file_extension : &str) -> bool{
        is_can_fill_forms(file_extension) || is_can_view(file_extension) || is_can_edit(file_extension) || is_can_convert(file_extension)
    }

    pub fn get_internal_extension(file_type : &str) -> String{
        let mapping = HashMap::from([
            ("word","docx"),
            ("cell","xlsx"),
            ("slide","pptx"),
            ("docxf","docxf")
        ]);

        mapping.get(file_type).map_or("docx".to_owned(), |f| f.to_string())
    }

    pub fn get_template_image_url(filetype : &str) -> String{
        let path = format!("{}{}",get_server_url(),"/static/images/");
        let mapping = HashMap::from([
            ("word", format!("{}{}",path,"file_docx.svg")),
            ("cell", format!("{}{}",path,"file_xlsx.svg")),
            ("slide",format!("{}{}",path,"file_pptx.svg")),
        ]);
        mapping.get(filetype).map_or("file_docx.svg".to_string(), |f| f.to_string())
    }
    /// Возвращает корректное имя файла
    /// 
    /// 
    /// 
    /// # Пример:
    ///     let name = get_correct_name("test.docx");
    ///     
    ///     // Если уже существует то вернет
    ///     name == "test (1).docx"
    /// 
    /// 
    /// 
    pub fn get_correct_name(filename : &str) -> Result<String>{ 
        let t_path = Path::new(filename);
        let basename = file_utils::get_file_name_no_ext(t_path)?;
        let ext = file_utils::get_file_ext(t_path)?;

        let mut name = format!("{basename}.{ext}");
        let mut i = 1;

        while std::path::Path::new(&name).exists() {
            name = format!("{basename}({i}){ext}");
            i +=1;
        }
        log::debug!("get_correct_name: {name}");
        Ok(name)
    }

    pub fn get_server_url() -> String{
        DOCUMENT_SERVER_URL.to_owned()
    }
    /// Возвращает корректный url для данного файла
    /// 
    /// ```
    ///     let file = "aboba.docx"
    /// 
    ///     let url = get_file_uri(file,8);
    /// 
    ///     url == "http://localhost:8080/static/8/aboba.docx"
    /// 
    /// ``` s
    pub fn get_file_uri(filename : &str, user_id : i32)-> String{
        let host = get_server_url();
        format!("{}{}/{}/{}",host,STATIC_URL,user_id,filename)
    }
    /// Возвращает путь к папке пользователя
    /// 
    /// Если папки нет то создает
    pub async fn get_root_folder(user_id : i32) -> PathBuf{
        let user_folder = format!("{}{}",ROOT_FOLDER,user_id);
        log::debug!("Folder {}", user_folder);
        let user_dir = Path::new(&user_folder);
        match user_dir.exists() {
            true => user_dir.to_path_buf(),
            false => {
                tokio::fs::create_dir(&user_folder).await.unwrap();
                user_dir.to_path_buf()
            },
        }
    }

    /// Возвращает путь к конкретной версии файла
    pub async fn get_history_path(filename : &str,file : &str, version : &str, user_id : i32) -> PathBuf{
        let storage = get_root_folder(user_id).await;
        storage.join( format!("{}-hist/{}/{}",filename,version,file))
       
    }
    /// Возвращает путь к данному файлу
    pub async fn get_storage_path(filename : &str,user_id : i32) -> PathBuf{
        let user_folder = get_root_folder(user_id).await;
        log::debug!("get_storage_path user_folder : {:?}", user_folder);

        log::debug!("get_storage_path filename : {:?}", filename);
        //PathBuf::from(format!("{}/{}",user_folder.to_string_lossy(),filename))
        user_folder.join(filename)
    }

    pub async fn get_forcesave_path(filename : &str,user_id : i32,create : bool) -> Option<PathBuf>{
        let user_folder = get_root_folder(user_id).await;
        let history_dir = user_folder.join(format!("{filename}-hist"));
        if !history_dir.exists() && create{
            tokio::fs::File::create(&history_dir).await;
            Some(history_dir)
        }
        else {
            None
        }

        
    }

    pub async fn create_file_response(mut response : reqwest::Response, path : &Path) -> Result<()>{
        if response.status() != StatusCode::OK{
            return Err(Error::DocManager);
        }
        let mut file = tokio::fs::File::create(path).await.unwrap();

        while let Some(chunk) = response.chunk().await.unwrap(){
            file.write_all(&chunk).await;
        }
        Ok(())
    }

    pub async fn save_file_from_uri(uri : &str, path : &Path){
        let resp = reqwest::get(uri).await.unwrap();
        create_file_response(resp, path).await.unwrap();
    }

    pub async fn create_file<T>(stream : &mut T,path : &Path, meta : bool)
        where 
        T : std::marker::Unpin  + AsyncRead + AsyncReadExt
    {
        use tokio::fs::File;
        use tokio::io::{AsyncWriteExt,AsyncReadExt};
        let mut out = File::create(path).await.unwrap();
        let mut buf  = [0u8;8192];
        while let Ok(b) = stream.read(&mut buf).await {
            out.write_all(&buf).await;
        }
    }

    /// Создает файл из assets
    pub async fn create_sample(filetype : &str, is_sample : bool, user_id : i32) -> Result<String>{
        let ext = get_internal_extension(filetype);
        let mut sample_name = String::from("new");
        if is_sample{
            sample_name = "sample".to_string();
        }
        let filename = get_correct_name(&format!("{sample_name}{ext}"))?;
        let path = get_storage_path(&filename,user_id).await;
        let mut asset_file = tokio::fs::File::open(
            format!("assets/{}/{}",sample_name,filename)
        ).await.unwrap();
        create_file(&mut asset_file, &path, true).await;
        Ok(filename)
    }

    pub async fn remove_file(filename : &str, user_id : i32){
        let path = get_storage_path(filename, user_id).await;
        let path = std::path::Path::new(&path);
        if path.exists(){
            // По идее должно быть безопасно, т.к путь точно существует
            // хотя может зафейлить во время конкретно удаления
            tokio::fs::remove_file(path).await.unwrap();
        }
        
    }

    pub async fn generate_file_key(filename : &str, user_id : i32) -> String{
        let path = get_storage_path(filename, user_id).await;
        let uri = get_file_uri(filename,user_id);
        let file = tokio::fs::File::open(path).await.unwrap();
        let metadata = file.metadata().await.unwrap();
        let last_mofif : u64 = metadata.modified().unwrap().duration_since(metadata.created().unwrap()).unwrap().as_secs();
        let input = format!("{}_{}",uri,last_mofif);
        let hash = xxhash_rust::xxh64::xxh64(input.as_bytes(), 1).to_string();
        hash[..19].to_string()
    }

    pub async fn get_js_scripts() -> Result<String>{
        use reqwest;
        let script = reqwest::get(format!("{}{}",get_server_url(),DOC_SERV_API_URL)).await?;
        Ok(script.text().await?)
    }

    pub async fn get_file_for_user(filename : &str, user_id : i32) -> Result<Vec<u8>>{
        let storage = &get_storage_path(filename, user_id).await;
        let file = Path::new(&storage);

        if !file.exists(){return Err(Error::DocManager)}

        Ok(tokio::fs::read(file).await.map_err(|_| Error::DocManager).unwrap())
    }
    /// 
    pub fn generate_revision_id(expected : &str) -> String{
        let mut key = expected.to_string();
        if key.len() > 20{
            key = xxhash_rust::xxh3::xxh3_64(expected.as_bytes()).to_string()
        }
        key[..19].to_string()
    }

    pub async fn download(file_path : &Path) -> Result<HttpResponse>{
        let mut file = tokio::fs::File::open(&file_path).await.map_err(|_| crate::error::Error::DocManager)?;
        let name = crate::office_utils::file_utils::get_file_name(file_path)?;
        let filesize = file.metadata().await.unwrap().len();
        let mut headers = HeaderMap::new();
        headers.append(header::CONTENT_LENGTH, HeaderValue::from_str(&filesize.to_string()).unwrap());
        headers.append(header::CONTENT_DISPOSITION,HeaderValue::from_str(&format!("attachment;filename={}",name)).unwrap());

        let mut buffer : Vec<u8> = Vec::with_capacity(81292);
        let bytes = file.read_to_end(&mut buffer).await.unwrap();
        Ok(HttpResponse::Ok().body(buffer))
    }
#[cfg(test)]

mod tests{
    use super::*;
    #[test]
    fn test_is_can(){
        let good_convert = "docm";
        let good = "docx";
        let wrong = "idk";
        assert_eq!(is_can_convert(good_convert),true);
        assert_eq!(is_can_convert(wrong),false);
        assert_eq!(is_can_edit(good),true);
        assert_eq!(is_can_edit(wrong),false);
        assert_eq!(is_can_fill_forms(good),true);
        assert_eq!(is_can_fill_forms(wrong),false);
        assert_eq!(is_can_view("pdf"),true);
        assert_eq!(is_can_view(wrong),false);
        assert_eq!(is_supported_extension(good),true);
        assert_eq!(is_supported_extension(wrong),false);
    }

    #[tokio::test]
    async fn test_generate_file_key() {
        let key1 = generate_file_key("test.txt", 8).await;
        let key2 = generate_file_key("test.txt", 8).await;
        assert_eq!(key1,key2);
    }
    #[tokio::test]
    async fn test_get_js(){
        let js = get_js_scripts().await.unwrap();  
        println!("{}",js);
    }
    #[tokio::test]
    async fn test_remove_file(){
        remove_file("test.txt", 8).await;
    }
}