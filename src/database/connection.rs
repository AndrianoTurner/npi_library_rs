use chrono::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use diesel::result;
use super::models::{self, User,NewUser};
use crate::schema::user_table::dsl::*;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Database{
    pool : DbPool,
}



type Result<T> = std::result::Result<T,diesel::result::Error>;

impl Database{
    pub fn new() -> Self{
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("Database URL is not set!");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool : DbPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to build pool!");
        Database { pool }
    }

    pub fn get_all_users(&self) -> Vec<User>{
        let conn: &mut PgConnection = &mut self.pool.get().unwrap();
        user_table.load::<User>(conn)
        .expect("Failed to load users from pool!")
    }

    pub fn create_user(&self, user : NewUser) -> Result<User>{
        let conn: &mut PgConnection = &mut self.pool.get().unwrap();
        diesel::insert_into(user_table)
            .values(&user)
            .get_result::<User>(conn)
    }

    pub fn get_user_by_id(&self, user_id : i32) -> Option<User>{
        let conn: &mut PgConnection = &mut self.pool.get().unwrap();
        user_table.find(user_id).get_result::<User>(conn).ok()
    }

    pub fn get_user_by_email(&self, other_email : String) -> Option<User>{
        let conn: &mut PgConnection = &mut self.pool.get().unwrap();
        user_table.filter(email.eq(other_email)).get_result(conn).ok()
    }

    pub fn delete_user_id(&self, user_id : i32 , user : NewUser ) -> Option<User>{
        let conn: &mut PgConnection = &mut self.pool.get().unwrap();
        diesel::update(user_table.find(user_id)).set(&user)
            .get_result::<User>(conn).ok()
    }
}