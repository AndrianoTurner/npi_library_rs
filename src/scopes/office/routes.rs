
use actix_web::{
    get,
    post, HttpResponse,
    web::{self, Payload, ServiceConfig}, HttpRequest, http::header::CONTENT_LENGTH,
};
use actix_multipart::{
    Multipart
};
use serde::{Serialize, Deserialize};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use crate::{auth::AuthenticationToken, State, office_utils::doc_manager::DocumentManager};
use futures_util::TryStreamExt;
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
            let destination = state.document_manager.get_storage_path(
                &field.content_disposition().get_filename().unwrap(), 
                user.id
            );

            let mut saved_file = fs::File::create(&destination).await.unwrap();
            while let Ok(Some(chunk)) = field.try_next().await{
                let _ = saved_file.write_all(&chunk).await.unwrap();
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
    state.document_manager.create_sample(&filetype, sample, auth.id).await;
    HttpResponse::Created().into()
}

pub fn build_routes(cfg : &mut ServiceConfig){
    cfg.service(upload);
    cfg.service(create_new);
}