use actix_web::{ResponseError, HttpResponse};



#[derive(thiserror::Error,Debug,PartialEq)]
pub enum Error{
    #[error("Database error {0}")]
    Database(String),
    #[error("Converter error {0}")]
    Converter(String),
    #[error("Track Manager error")]
    Track,
    #[error("File Utils error")]
    FileUtils,
    #[error("Doc Manager Error")]
    DocManager,
}

impl From<reqwest::Error> for Error{
    fn from(val: reqwest::Error) -> Self {
        Self::Converter(val.to_string())
    }
}

impl From<sqlx::Error> for Error{
    fn from(val: sqlx::Error) -> Self {
        Self::Database(val.to_string())
    }
}


impl ResponseError for Error{
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self{
            Error::Converter(_) => HttpResponse::InternalServerError().json("Something went wrong converting"),
            Error::Database(_) => HttpResponse::InternalServerError().json("Something went wrong database"),
            Error::Track => HttpResponse::InternalServerError().json("Something went wrong tracking"),
            Error::DocManager => HttpResponse::InternalServerError().json("Something went wrong docmanager"),
            Error::FileUtils => HttpResponse::InternalServerError().json("Something wrong fileutils")
        }
        
    }
}
