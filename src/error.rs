use actix_web::{ResponseError, HttpResponse};



#[derive(thiserror::Error,Debug,PartialEq)]
pub enum Error{
    #[error("Database error")]
    DatabaseError,
    #[error("Converter error")]
    ConverterError,
    #[error("Track Manager error")]
    TrackError,
    #[error("File Utils error")]
    FileUtilsError,
    #[error("Doc Manager Error")]
    DocManagerError,
}

impl From<reqwest::Error> for Error{
    fn from(_: reqwest::Error) -> Self {
        Self::ConverterError
    }
}

impl From<sqlx::Error> for Error{
    fn from(_: sqlx::Error) -> Self {
        Self::DatabaseError
    }
}


impl ResponseError for Error{
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self{
            Error::ConverterError => HttpResponse::InternalServerError().json("Something went wrong converting"),
            Error::DatabaseError => HttpResponse::InternalServerError().json("Something went wrong database"),
            Error::TrackError => HttpResponse::InternalServerError().json("Something went wrong tracking"),
            Error::DocManagerError => HttpResponse::InternalServerError().json("Something went wrong docmanager"),
            Error::FileUtilsError => HttpResponse::InternalServerError().json("Something wrong fileutils")
        }
        
    }
}
