use actix_web::{ResponseError, HttpResponse};



#[derive(thiserror::Error,Debug,PartialEq)]
pub enum Error{
    #[error("Database error")]
    Database,
    #[error("Converter error")]
    Converter,
    #[error("Track Manager error")]
    Track,
    #[error("File Utils error")]
    FileUtils,
    #[error("Doc Manager Error")]
    DocManager,
}

impl From<reqwest::Error> for Error{
    fn from(_: reqwest::Error) -> Self {
        Self::Converter
    }
}

impl From<sqlx::Error> for Error{
    fn from(_: sqlx::Error) -> Self {
        Self::Database
    }
}


impl ResponseError for Error{
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self{
            Error::Converter => HttpResponse::InternalServerError().json("Something went wrong converting"),
            Error::Database => HttpResponse::InternalServerError().json("Something went wrong database"),
            Error::Track => HttpResponse::InternalServerError().json("Something went wrong tracking"),
            Error::DocManager => HttpResponse::InternalServerError().json("Something went wrong docmanager"),
            Error::FileUtils => HttpResponse::InternalServerError().json("Something wrong fileutils")
        }
        
    }
}
