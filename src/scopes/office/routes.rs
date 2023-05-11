#![allow(non_snake_case,unused,dead_code)]
use actix_web::{
    get,
    post, HttpResponse,
    web::{self, ServiceConfig}, HttpRequest, http::header::CONTENT_LENGTH,
};
use actix_multipart::{
    Multipart
};
use serde::{Serialize, Deserialize};
use tokio::{fs};
use tokio::io::AsyncWriteExt;
use crate::{auth::AuthenticationToken, State, office_utils::{file_utils, doc_manager}};
use futures_util::{TryStreamExt};
use crate::office_utils::models::{CallbackData};
use crate::office_utils::doc_manager::*;
#[post("/upload")]
pub async fn upload(mut payload : Multipart,state : web::Data<State>, auth : AuthenticationToken, req: HttpRequest ) -> HttpResponse{
    let max_file_size = 10_000;
    let max_file_count = 1;
    let content_length = match req.headers().get(CONTENT_LENGTH) {
        Some(hv) => hv.to_str().unwrap_or("0").parse().unwrap(),
        None => 0,
    };

    if content_length.eq(&0) || content_length > max_file_size {
        return HttpResponse::BadRequest().into();
    }
    let mut current_count = 0;
    // Unwrap должен быть безопасен, т.к. проверка на валидность токена проведена
    let user = state.database.get_user_by_id(auth.id).await.unwrap();
    loop{
        if current_count >= max_file_count {break;}
        if let Ok(Some(mut field)) = payload.try_next().await{
            let filetype = field.content_type();

            if filetype.is_none() {continue;};
            let destination = get_storage_path(
                field.content_disposition().get_filename().unwrap(), 
                user.id
            ).await;

            let mut saved_file = fs::File::create(&destination).await.unwrap();
            while let Ok(Some(chunk)) = field.try_next().await{
                saved_file.write_all(&chunk).await.unwrap();
            }
        }
        current_count +=1;
    }
    HttpResponse::Ok().into()
}
#[derive(Serialize,Deserialize)]
pub struct CreateFileInfo{
    file_type : String,
    sample : bool,
}

#[get("/create")]
pub async fn create_new(req : HttpRequest, state : web::Data<State>,create_file_info : web::Json<CreateFileInfo>, auth : AuthenticationToken ) -> HttpResponse{
    let info = create_file_info.into_inner();
    if info.file_type.is_empty() {return HttpResponse::BadRequest().json("Wrong filetype!");}
    let filetype = info.file_type;
    let sample = info.sample;
    create_sample(&filetype, sample, auth.id).await;
    HttpResponse::Created().into()
}

#[get("/load-api.js")]
pub async fn load_js(state : web::Data<State>) -> HttpResponse{
    let js = get_js_scripts().await.unwrap();
    HttpResponse::Ok().content_type(mime::APPLICATION_JAVASCRIPT_UTF_8).body(js)
}
#[post("/track")]
pub async fn track(data : web::Json<CallbackData>, state : web::Data<State>) -> Result<HttpResponse, Box<dyn std::error::Error>>{
    use futures_util::stream::{TryStreamExt};
    use tokio_util::io::StreamReader;
    let data = data.into_inner();
    log::debug!("Status: {:?}",data);

    #[derive(Serialize)]
    struct Response{
        error : i32,
    }
    if data.status == 1{
        if let Some(actions) = data.actions{
            if actions[0]._type == 0{
                let user = &actions[0].userid;
            }
        }
    }
    if data.status == 2 && data.url.is_some(){
            let url = data.url.unwrap();
            let resp = reqwest::get(&url).await?;
            let filename = get_correct_name(&url)?;
            let path = get_storage_path(&filename,8).await;
            let stream = resp.bytes_stream()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other,e));
            let mut stream_reader = StreamReader::new(stream);
            create_file(&mut stream_reader, &path, false).await;
            return Ok(HttpResponse::Ok().json(Response {error : 0}));
        }
        Ok(HttpResponse::Ok().json(Response {error : 0}))
    }
    

#[get("/file/{filename}")]
pub async fn get_file(state : web::Data<State>) -> HttpResponse{
    let bytes = get_file_for_user("test.docx", 8).await.unwrap();
    HttpResponse::Ok().content_type("application/octet-stream").body(bytes)
}
#[get("/download/{user_id}/{filename}")]
pub async fn download(path : web::Path<(i32,String)>) -> actix_web::Result<HttpResponse>{
    let path = path.into_inner();
    let (user_id,filename) = {(path.0,path.1)};
    let filename = file_utils::get_file_name(&filename)?;

    let file_path = doc_manager::get_storage_path(&filename, user_id).await;

    let response = doc_manager::download(&file_path).await?;

    Ok(response)


}
pub fn build_routes(cfg : &mut ServiceConfig){
    cfg.service(upload);
    cfg.service(create_new);
    cfg.service(load_js);
    cfg.service(get_file);
    cfg.service(track);
    cfg.service(download);
}