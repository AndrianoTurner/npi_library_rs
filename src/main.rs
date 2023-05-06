use actix_web::{App,HttpServer, middleware, web};
use dotenvy;
use env_logger;
mod database;
mod repositories;
mod models;
use database::connection::Database;
struct State{
    database : Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenvy::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let database = Database::new().await;
    HttpServer::new(move ||{
        App::new()
            .app_data(web::Data::new(State{database : database.clone()}))
            .wrap(middleware::Logger::default())
            .service(web::scope("/api/user").configure(repositories::user_repository::routes::build_routes))
    }).bind("127.0.0.1:8080")?
    .run()
    .await
}