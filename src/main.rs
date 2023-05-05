use actix_web::{App,main,HttpServer, middleware, web};
use dotenv;
use env_logger;
mod api;
mod database;
mod repositories;
mod models;
mod schema;
use database::connection::Database;
struct State{
    database : Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    env_logger::init();
    dotenv::dotenv().unwrap();
    let database = Database::new();
    HttpServer::new(move ||{
        App::new()
            .app_data(web::Data::new(database.clone()))
            .wrap(middleware::Logger::default())
    }).bind("127.0.0.1:8080")?
    .run()
    .await
}