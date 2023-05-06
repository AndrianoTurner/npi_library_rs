use actix_web::{ResponseError, HttpResponse};
use serde::{Serialize,Deserialize};

#[derive(Deserialize,Serialize,Debug)]
pub struct LoginRequest{
    pub email : String,
    pub password : String,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct RegisterRequest{
    pub email : String,
    pub password : String,
}
#[derive(Debug,Clone)]
pub struct ValidationError;

impl std::fmt::Display for ValidationError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f," couldn't validate form!")
    }
}

impl ResponseError for ValidationError{
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::new(self.status_code())
    }
}

pub trait ValidateForm{
    fn validate(&self) -> Result<(),ValidationError>;
}


impl ValidateForm for RegisterRequest{
    fn validate(&self) -> Result<(),ValidationError> {
        if self.email.trim().len().eq(&0) || self.password.trim().len().eq(&0){
            return Err(ValidationError)
        }
        Ok(())
    }
}

impl ValidateForm for LoginRequest{
    fn validate(&self) -> Result<(),ValidationError> {
        if self.email.trim().len().eq(&0) || self.password.trim().len().eq(&0){
            return Err(ValidationError)
        }
        Ok(())
    }
}