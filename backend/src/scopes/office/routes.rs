#![allow(non_snake_case, unused, dead_code)]
use crate::office_utils::doc_manager::*;
use crate::office_utils::models::CallbackData;
use crate::scopes::user;
use crate::{
    auth::AuthenticationToken,
    office_utils::{doc_manager, file_utils, track_manager},
    State,
};
use actix_multipart::Multipart;
use actix_web::{
    get,
    http::header::CONTENT_LENGTH,
    post,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse,
};
use futures_util::TryStreamExt;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::io::AsyncWriteExt;
#[post("/upload")]
pub async fn upload(
    mut payload: Multipart,
    state: web::Data<State>,
    auth: AuthenticationToken,
    req: HttpRequest,
) -> HttpResponse {
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
    loop {
        if current_count >= max_file_count {
            break;
        }
        if let Ok(Some(mut field)) = payload.try_next().await {
            let filetype = field.content_type();

            if filetype.is_none() {
                continue;
            };
            let destination =
                get_storage_path(field.content_disposition().get_filename().unwrap(), user.id)
                    .await;

            let mut saved_file = fs::File::create(&destination).await.unwrap();
            while let Ok(Some(chunk)) = field.try_next().await {
                saved_file.write_all(&chunk).await.unwrap();
            }
        }
        current_count += 1;
    }
    HttpResponse::Ok().into()
}
#[derive(Serialize, Deserialize)]
pub struct CreateFileInfo {
    file_type: String,
    sample: bool,
}

#[get("/create")]
pub async fn create_new(
    req: HttpRequest,
    state: web::Data<State>,
    create_file_info: web::Json<CreateFileInfo>,
    auth: AuthenticationToken,
) -> HttpResponse {
    let info = create_file_info.into_inner();
    if info.file_type.is_empty() {
        return HttpResponse::BadRequest().json("Wrong filetype!");
    }
    let filetype = info.file_type;
    let sample = info.sample;
    create_sample(&filetype, sample, auth.id).await;
    HttpResponse::Created().into()
}

#[post("/track/{user_id}/{filename}")]
pub async fn track(
    data: web::Json<CallbackData>,
    state: web::Data<State>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    use futures_util::stream::TryStreamExt;
    use tokio_util::io::StreamReader;
    let data = data.into_inner();
    let (user_id, filename) = path.into_inner();
    let user_id = user_id.parse::<i32>().unwrap_or(-1);
    log::debug!("Status: {:?}", data);

    #[derive(Serialize)]
    struct Response {
        error: i32,
    }
    let filename = file_utils::get_file_name(&std::path::PathBuf::from(&filename)).unwrap();
    if data.status == 1 {
        if let Some(actions) = &data.actions {
            if actions[0]._type == 0 {
                let user = &actions[0].userid;
                track_manager::command_request("forcesave", &data.key, None).await
            }
        }
    }
    if data.status == 2 || data.status == 3 && data.url.is_some() {
        log::debug!("/track process save called!");
        track_manager::process_save(&data, &filename, user_id).await?;
    }
    if data.status == 6 || data.status == 7 {
        track_manager::process_force_save(data, &filename, user_id);
    }

    Ok(HttpResponse::Ok().json(Response { error: 0 }))
}

#[get("/download/{user_id}/{filename}")]
pub async fn download(path: web::Path<(i32, String)>) -> actix_web::Result<HttpResponse> {
    let path = path.into_inner();
    let (user_id, filename) = { (path.0, path.1) };
    let filename = file_utils::get_file_name(std::path::Path::new(&filename))?;

    let file_path = doc_manager::get_storage_path(&filename, user_id).await;
    log::debug!("download : file_path {:?}", file_path);
    let response = doc_manager::download(&file_path).await?;
    log::debug!("download : response {:?}", response);
    Ok(response)
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
struct BooksResponse {
    books: Option<Vec<String>>,
}

pub fn build_routes(cfg: &mut ServiceConfig) {
    cfg.service(upload);
    cfg.service(create_new);
    cfg.service(track);
    cfg.service(download);
}
