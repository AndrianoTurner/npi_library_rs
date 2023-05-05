use serde::{Serialize,Deserialize};
use diesel::{Queryable,Insertable,AsChangeset};
#[derive(Deserialize,Serialize,Debug,Queryable)]
pub struct User{
    pub id : i32,
    pub email : String,
    pub password : String,
    pub first_name : Option<String>,
    pub second_name : Option<String>,
    pub last_name : Option<String>,
    
}
#[derive(Insertable,AsChangeset)]
#[diesel(table_name = crate::schema::user_table)]
pub struct NewUser{
    pub email : String,
    pub password : String,
    pub first_name : Option<String>,
    pub second_name : Option<String>,
    pub last_name : Option<String>,
}

impl NewUser{
    pub fn new(email : &String, pass: &String) -> Self{
        Self { email : email.clone(),
            password : pass.clone(),
            first_name : None,
            second_name : None,
            last_name : None }
    }
}
