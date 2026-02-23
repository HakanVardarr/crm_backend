use bigdecimal::BigDecimal;
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

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize)]
pub struct CustomerDetail {
    pub customer_info: Customer,
    pub customer_notes: Vec<CustomerNote>,
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

#[derive(Serialize)]
pub struct PaginatedCustomers {
    pub data: Vec<Customer>,
    pub total_count: i64,
    pub page: i64,
    pub total_pages: i64,
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

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "cephe_enum")]
pub enum Cephe {
    #[sqlx(rename = "KB")]
    KB,
    #[sqlx(rename = "KD")]
    KD,
    #[sqlx(rename = "GB")]
    GB,
    #[sqlx(rename = "GD")]
    GD,
    #[sqlx(rename = "KB-KD")]
    KbKd,
    #[sqlx(rename = "KD-KB")]
    KdKb,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Property {
    pub id: Uuid,
    pub daire_no: String,
    pub blok: String,
    pub kat: String,
    pub kapi_no: i32,
    pub daire_tipi: String,
    pub oda_sayisi: String,
    pub brut_m2: BigDecimal,
    pub net_m2: BigDecimal,
    pub balkon_m2: Option<BigDecimal>,
    pub cephe: Option<Cephe>,
    pub kiraci_var_mi: bool,
    pub sahip_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct CreateProperty {
    pub daire_no: String,
    pub blok: String,
    pub kat: String,
    pub kapi_no: i32,
    pub daire_tipi: String,
    pub oda_sayisi: String,
    pub brut_m2: BigDecimal,
    pub net_m2: BigDecimal,
    pub balkon_m2: Option<BigDecimal>,
    pub cephe: Option<Cephe>,
    pub kiraci_var_mi: bool,
    pub sahip_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct PaginatedProperties {
    pub data: Vec<Property>,
    pub total_count: i64,
    pub page: i64,
    pub total_pages: i64,
}
