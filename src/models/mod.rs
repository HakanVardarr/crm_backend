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

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub is_admin: bool,
    pub exp: usize,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Customer {
    pub id: Uuid,
    pub ad_soyad: String,
    pub gsm: Option<String>,
    pub telefon: Option<String>,
    pub email: Option<String>,
    pub acil_kisi: Option<String>,
    pub uyruk: Option<String>,
    pub en_son_gorusuldu: Option<chrono::DateTime<chrono::Utc>>,
    pub danisan_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct CreateCustomer {
    pub ad_soyad: String,
    pub gsm: Option<String>,
    pub telefon: Option<String>,
    pub email: Option<String>,
    pub acil_kisi: Option<String>,
    pub uyruk: Option<String>,
    pub en_son_gorusuldu: Option<chrono::DateTime<chrono::Utc>>,
    pub danisan_id: Option<Uuid>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct CustomerNote {
    pub id: Uuid,
    pub note: String,
    pub created_by: Uuid,
    pub customer_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct CreateCustomerNote {
    pub note: String,
    pub created_by: Uuid,
}
