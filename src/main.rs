use actix_web::{App,HttpServer, middleware, web};
use config::ROOT_FOLDER;
mod database;
mod scopes;
mod models;
mod auth;
mod office_utils;
use database::connection::Database;
mod config;
mod error;
pub struct State{
    database : Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenvy::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let app_data = std::path::Path::new(ROOT_FOLDER);
    if !app_data.exists(){
        match std::fs::create_dir(app_data){
            Ok(_) => (),
            Err(e) => match e.kind(){
                std::io::ErrorKind::AlreadyExists => (),
                _ => panic!("Failed to create app_data dir")
            }
        }
    }
    let database = Database::new().await;
    HttpServer::new(move ||{
        App::new()
            .app_data(web::Data::new(State{database : database.clone()}))
            .wrap(middleware::Logger::default())
            .service(web::scope("/api")
                .service(web::scope("/user").configure(scopes::user::routes::build_routes))
                .service(web::scope("/office").configure(scopes::office::routes::build_routes)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}