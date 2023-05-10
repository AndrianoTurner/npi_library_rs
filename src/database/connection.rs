#![allow(non_snake_case,unused,dead_code)]
use dotenvy::dotenv;
use super::models::{User, hash_password};
use sqlx::postgres::{PgPoolOptions,PgPool};
use sqlx::Result;

#[derive(Clone)]

/// Структура, содержащая функции для взаимодействия с БД
pub struct Database{
    pool : PgPool,
}

impl Database{
    /// Конструктор структуры БД
    /// 
    /// 
    /// 
    /// Может **паниковать**, если не удалось соединиться с БД.
    pub async fn new() -> Self{
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("Database URL is not set!");
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await.expect("Failed to connect to db!"); 
            // Expect уместен, т.к. если бд не запущена то и 
            // приложение работать не должно
        Database { pool }
    }

    /// Функция возвращающая всех пользователей
    /// 
    /// 
    /// На данный момент может **паниковать**
    pub async fn get_all_users(&self) -> Vec<User>{
        let query = "SELECT * FROM user_table";
        sqlx::query_as( query)
        .fetch_all(&self.pool)
        .await.unwrap()
    }
    
    /// Функция, создающая пользователя
    /// 
    pub async fn create_user(&self, email : &str, password : &str) -> Result<()>{
        log::debug!("{} {}",email,password);
        let query = "INSERT INTO user_table (email,password) VALUES ($1,$2)";


        // TODO Блокировать поток, т.к. функция блокирующая
        let password = hash_password(password);
        sqlx::query(query)
            .bind(email)
            .bind(&password)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_user_by_id(&self, user_id : i32) -> Option<User>{
        let query = "SELECT * FROM user_table WHERE id = $1";
        sqlx::query_as(query)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
            .ok()
    }

    pub async fn get_user_by_email(&self, email : &str) -> Result<User>{
        let query = "SELECT * FROM user_table WHERE email = $1";
        let a = sqlx::query_as(query)
            .bind(email)
            .fetch_one(&self.pool)
            .await;

        log::debug!("User {:?}",a);
        a
        
    }

    pub async fn delete_user_id(&self, user_id : i32) -> Result<()>{
        let query = "DELETE FROM user_table where id = $1";
        sqlx::query(query)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_user(&self, user_id : i32, email : &str, password : &str) -> Result<()>{
        let query = "UPDATE user_table SET email = $1, password = $2 WHERE id = $3";
        sqlx::query(query)
            .bind(email)
            .bind(password)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    
    }

    pub async fn update_user_email(&self, user_id : i32, email : &str) -> Result<()>{
        let query = "UPDATE user_table SET email = $1 WHERE id = $2";
        sqlx::query(query)
            .bind(email)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_user_password(&self,user_id : i32, password : &str) -> Result<()>{
        let query = "UPDATE user_table SET password = $1 WHERE id = $2";
        sqlx::query(query)
            .bind(password)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
