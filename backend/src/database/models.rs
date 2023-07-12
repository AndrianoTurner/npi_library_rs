#![allow(non_snake_case,unused)]

use std::str::FromStr;

use serde::{Serialize,Deserialize};
use argon2::{
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use log::debug;
use sqlx::{FromRow, Row};

use crate::office_utils::models::Document;
const SALT : &str = "YWJvYmExMjM";
#[derive(Serialize,Deserialize,Debug,FromRow)]
pub struct Role{
    id : String,
}
#[derive(Serialize,Deserialize,Debug,FromRow)]
pub struct Permission{
    id : String,
}
#[derive(Serialize,Deserialize,Debug,FromRow)]
pub struct RolePermission{
    role_id : String,
    permission_id : String,
}
#[derive(Serialize,Deserialize,Debug,FromRow)]
pub struct UserRole{
    user_id : i32,
    role_id : String,
}



#[derive(Deserialize,Serialize,Debug,PartialEq,sqlx::FromRow)]
pub struct User{
    pub id : i32,
    pub email : String,
    pub password : String,
}

#[derive(Debug,Deserialize,Serialize,Clone,sqlx::FromRow)]
pub struct Book{
    id : i32,
    owner_id : i32,
    filepath : String,
    filename : String,
}


impl Book{
    pub fn construct_link(&self) -> String{
        format!("download/{}/{}",self.owner_id,self.filepath)
    }
}

impl User{
    pub fn check_password(&self,other_password : &str) -> bool{
        let argon = Argon2::default();
        debug!("PASSWORD: {}",self.password);
        if let Ok(pass) = PasswordHash::new(&self.password){
            return argon.verify_password(other_password.as_bytes(), &pass).is_ok();
        }
        false
    }
    // Данная штука нужна для имитации проверки пароля, если были введены неверные данные
    // чтобы исключить проверку на существует ли пользователь или нет
    /// Функция, вычисляющая хэш несуществующего пользователя
    pub fn mock_user_password(){
        let u = User{
            id : 0, 
            email : "abob@mail.xyz".to_string(), 
            password : "$argon2id$v=19$m=19456,t=2,p=1$YWJvYmExMjM$Kvd5Dp+uzp2Ycm07bLIB+nr7UOm0GPQ9Z2tV7Q58fHE".to_string(),
        };
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


