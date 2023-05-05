use serde::{Serialize,Deserialize};



#[derive(Deserialize,Serialize,Debug)]
pub struct LoginInfo{
    email : String,
    password : String,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct Claims{
    pub sub : String,
    pub exp : usize,

}
#[derive(Deserialize,Serialize,Debug)]
pub struct Register{
    pub email : String,
    pub password : String,
}