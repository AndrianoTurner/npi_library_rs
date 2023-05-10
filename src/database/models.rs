#![allow(non_snake_case,unused)]

use serde::{Serialize,Deserialize};
use argon2::{
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use log::debug;
use sqlx::FromRow;

use crate::office_utils::models::Document;
const SALT : &str = "YWJvYmExMjM";
#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub enum Groups{
    None,
    Student,
    Teacher,
    Admin,
}

impl From<std::string::String> for Groups{
    fn from(value: String) -> Self {
        match value {
            a if a == "None" => Self::None,
            b if b == "Student" => Self::Student,
            c if c == "Teacher" => Self::Teacher,
            d if d == "Admin" => Self::Admin,
            _ => Self::None
        }
    }
}

impl std::fmt::Display for Groups{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Groups::None => write!(f,"None"),
            Groups::Student => write!(f,"Student"),
            Groups::Admin => write!(f,"Admin"),
            Groups::Teacher => write!(f,"Teacher"),
        }
    }
}

#[derive(Deserialize,Serialize,Debug,PartialEq,sqlx::FromRow)]
pub struct User{
    pub id : i32,
    pub email : String,
    pub password : String,
    pub group : String,
}

impl User{
    pub fn check_password(&self,other_password : &str) -> bool{
        let argon = Argon2::default();
        debug!("PASSWORD: {}",self.password);
        let pass = PasswordHash::new(&self.password).unwrap();
        argon.verify_password(other_password.as_bytes(), &pass).is_ok()
    }
    // Данная штука нужна для имитации проверки пароля, если были введены неверные данные
    // чтобы исключить проверку на существует ли пользователь или нет
    /// Функция, вычисляющая хэш несуществующего пользователя
    pub fn mock_user_password(){
        let u = User{id : 0, email : "abob@mail.xyz".to_string(), password : "$argon2id$v=19$m=19456,t=2,p=1$YWJvYmExMjM$Kvd5Dp+uzp2Ycm07bLIB+nr7UOm0GPQ9Z2tV7Q58fHE".to_string(), group : Groups::None.to_string()};
        u.check_password("12345678910123");
    }


    pub fn get_user_files() -> Vec<Document>{
        todo!()
    }
}
/// Функция хэширования пароля
pub fn hash_password(password : &str) -> String{
    let argon = Argon2::default();

    let salt =  SaltString::from_b64(SALT).unwrap();
    argon.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}


