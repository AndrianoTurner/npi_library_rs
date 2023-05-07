use actix_web::{App,HttpServer, middleware, web};
use dotenvy;
use env_logger;
mod database;
mod scopes;
mod models;
mod auth;
mod office_utils;
use database::connection::Database;
use office_utils::doc_manager::DocumentManager;
mod config;
pub struct State{
    database : Database,
    document_manager : DocumentManager,
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenvy::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let database = Database::new().await;
    HttpServer::new(move ||{
        App::new()
            .app_data(web::Data::new(State{database : database.clone(), document_manager : DocumentManager::new()}))
            .wrap(middleware::Logger::default())
            .service(web::scope("/api")
                .service(web::scope("/user").configure(scopes::user::routes::build_routes))
                .service(web::scope("/office").configure(scopes::office::routes::build_routes)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}