use serde::{Serialize,Deserialize};
use argon2::{
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use log::debug;
const SALT : &str = "YWJvYmExMjM";
pub enum Roles{
    Student,
    Teacher,
    Admin,
}

#[derive(Deserialize,Serialize,Debug,PartialEq,sqlx::FromRow)]
pub struct User{
    pub id : i32,
    pub email : String,
    pub password : String,
}

impl User{
    pub fn check_password(&self,other_password : &str) -> bool{
        let argon = Argon2::default();
        debug!("PASSWORD: {}",self.password);
        let pass = PasswordHash::new(&self.password).unwrap();
        match argon.verify_password(other_password.as_bytes(), &pass){
            Ok(_) => true,
            Err(_) => false,
        }
    }
    // Данная штука нужна для имитации проверки пароля, если были введены неверные данные
    // чтобы исключить проверку на существует ли пользователь или нет
    /// Функция, вычисляющая хэш несуществующего пользователя
    pub fn mock_user_password(){
        let u = User{id : 0, email : "abob@mail.xyz".to_string(), password : "$argon2id$v=19$m=19456,t=2,p=1$YWJvYmExMjM$Kvd5Dp+uzp2Ycm07bLIB+nr7UOm0GPQ9Z2tV7Q58fHE".to_string()};
        u.check_password("12345678910123");
    }
}
/// Функция хэширования пароля
pub fn hash_password(password : &str) -> String{
    let argon = Argon2::default();

    let salt =  SaltString::from_b64(SALT).unwrap();
    argon.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}


