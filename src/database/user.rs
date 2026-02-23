use crate::models::User;
use crate::models::{CreateUser, UserWithPassword};
use uuid::Uuid;

use crate::database::Database;

impl Database {
    pub async fn list_users(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT id, name, last_name, email, is_admin FROM users")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, name, last_name, email, is_admin FROM users WHERE id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn create_user(&self, body: &CreateUser) -> Result<User, sqlx::Error> {
        let password_hash = bcrypt::hash(&body.password, 12).unwrap();

        sqlx::query_as::<_, User>(
            "INSERT INTO users (name, last_name, email, password_hash, is_admin)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, name, last_name, email, is_admin",
        )
        .bind(&body.name)
        .bind(&body.last_name)
        .bind(&body.email)
        .bind(&password_hash)
        .bind(body.is_admin)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_user_by_email(
        &self,
        email: &str,
    ) -> Result<Option<UserWithPassword>, sqlx::Error> {
        sqlx::query_as::<_, UserWithPassword>(
            "SELECT id, email, password_hash, is_admin FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
    }
}
