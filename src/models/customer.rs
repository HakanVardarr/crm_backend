use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::Property;

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

#[derive(Serialize, sqlx::FromRow)]
pub struct CustomerWithProperties {
    pub id: Uuid,
    pub ad_soyad: String,
    pub gsm: Option<String>,
    pub telefon: Option<String>,
    pub email: Option<String>,
    pub acil_kisi: Option<String>,
    pub uyruk: Option<String>,
    pub en_son_gorusuldu: Option<chrono::DateTime<chrono::Utc>>,
    pub danisan_id: Option<Uuid>,
    pub property_count: i64,
    pub daire_nolar: Option<String>,
}

#[derive(Serialize)]
pub struct CustomerDetail {
    pub customer_info: Customer,
    pub customer_notes: Vec<CustomerNote>,
    pub customer_properties: Vec<Property>,
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
}
