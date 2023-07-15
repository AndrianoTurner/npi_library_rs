#![allow(non_snake_case, unused, dead_code)]
use crate::{office_utils::doc_manager::get_storage_path, scopes::user};

use super::models::*;
use dotenvy::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};

type MyResult<T> = std::result::Result<T, crate::error::Error>;
/// Структура, содержащая функции для взаимодействия с БД
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Конструктор структуры БД
    ///
    ///
    ///
    /// Может **паниковать**, если не удалось соединиться с БД.
    pub async fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("Database URL is not set!");
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .expect("Failed to connect to db!");
        // Expect уместен, т.к. если бд не запущена то и
        // приложение работать не должно
        Database { pool }
    }

    /// Функция возвращающая всех пользователей
    ///
    ///
    /// На данный момент может **паниковать**
    pub async fn get_all_users(&self) -> Option<Vec<User>> {
        let query = "SELECT * FROM user_table";
        sqlx::query_as(query).fetch_all(&self.pool).await.ok()
    }

    /// Функция, создающая пользователя
    ///
    pub async fn create_user(&self, email: &str, password: &str) -> MyResult<()> {
        log::debug!("{} {}", email, password);
        let query = "INSERT INTO user_table (email,password) VALUES ($1,$2)";
        let query2 = "INSERT INTO user_roles (user_id,role_id) VALUES ($1,$2)";
        let cloned_pass = password.to_string();
        let pass = tokio::task::spawn_blocking(move || hash_password(&cloned_pass))
            .await
            .unwrap();

        sqlx::query(query)
            .bind(email)
            .bind(&pass)
            .execute(&self.pool)
            .await;
        let user = self.get_user_by_email(email).await.unwrap();
        sqlx::query(query2)
            .bind(user.id)
            .bind("none".to_string())
            .execute(&self.pool)
            .await;
        Ok(())
    }

    pub async fn get_user_by_id(&self, user_id: i32) -> Option<User> {
        let query = "SELECT * FROM user_table WHERE id = $1";
        sqlx::query_as(query)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
            .ok()
    }

    pub async fn get_user_by_email(&self, email: &str) -> MyResult<User> {
        let query = "SELECT * FROM user_table WHERE email = $1";
        sqlx::query_as(query)
            .bind(email)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| crate::error::Error::Database(e.to_string()))
    }

    pub async fn delete_user_id(&self, user_id: i32) -> MyResult<()> {
        let query = "DELETE FROM user_table where id = $1";
        sqlx::query(query).bind(user_id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn update_user(&self, user_id: i32, email: &str, password: &str) -> MyResult<()> {
        let query = "UPDATE user_table SET email = $1, password = $2 WHERE id = $3";
        sqlx::query(query)
            .bind(email)
            .bind(password)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_user_email(&self, user_id: i32, email: &str) -> MyResult<()> {
        let query = "UPDATE user_table SET email = $1 WHERE id = $2";
        sqlx::query(query)
            .bind(email)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_user_password(&self, user_id: i32, password: &str) -> MyResult<()> {
        let query = "UPDATE user_table SET password = $1 WHERE id = $2";
        sqlx::query(query)
            .bind(password)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn has_permission(&self, user_id: i32, permission_id: &str) -> MyResult<bool> {
        let query = "SELECT * FROM permissions
                            INNER JOIN role_permissions
                                ON permissions.id = role_permissions.permission_id
                            WHERE
                                permissions.id = $2 AND
                                role_permissions.role_id IN (
                                    SELECT role_id
                                    FROM user_roles
                                    WHERE user_id = $1
                                );";
        let permsissions = sqlx::query(query)
            .bind(user_id)
            .bind(permission_id)
            .execute(&self.pool)
            .await?;
        if permsissions.rows_affected() == 0 {
            return Ok(false);
        } else {
            return Ok(true);
        }

        Ok(false)
    }

    pub async fn save_book(&self, filename: &str, user_id: i32, title: &str, discipline: &str) {
        let query = "INSERT INTO books (title,discipline,owner_id,filepath) VALUES ($1,$2,$3,$4)";
        let filepath = get_storage_path(filename, user_id).await;

        sqlx::query(query)
            .bind(title)
            .bind(discipline)
            .bind(user_id)
            .bind(filepath.to_str().unwrap())
            .execute(&self.pool)
            .await;
    }

    pub async fn get_all_books(&self) -> Vec<Book> {
        let query = "SELECT * from books";

        let books: Option<Vec<Book>> = sqlx::query_as(query).fetch_all(&self.pool).await.ok();

        match books {
            Some(books) => books,
            None => Vec::new(),
        }
    }
}
