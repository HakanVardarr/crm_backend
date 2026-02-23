use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(sqlx::FromRow)]
pub struct UserWithPassword {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub is_admin: bool,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub is_admin: bool,
}
