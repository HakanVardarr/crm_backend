use crate::models::{
    CreateCustomer, CreateCustomerNote, CreateUser, Customer, CustomerNote, UserWithPassword,
};

use super::models::User;
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Failed to connect to database.")]
    ConnectionFailed,
    #[error("Failed to migrate.")]
    MigrationFailed,
}

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, DatabaseError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(|_| DatabaseError::ConnectionFailed)?;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|_| DatabaseError::MigrationFailed)?;

        Ok(Self { pool })
    }

    pub async fn list_users(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT id, name, last_name, email, is_admin FROM users")
            .fetch_all(&self.pool)
            .await
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

    pub async fn create_customer(&self, body: &CreateCustomer) -> Result<Customer, sqlx::Error> {
        sqlx::query_as::<_, Customer>(
        "INSERT INTO customers (ad_soyad, gsm, telefon, email, acil_kisi, uyruk, en_son_gorusuldu, danisan_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING *"
        )
        .bind(&body.ad_soyad)
        .bind(&body.gsm)
        .bind(&body.telefon)
        .bind(&body.email)
        .bind(&body.acil_kisi)
        .bind(&body.uyruk)
        .bind(body.en_son_gorusuldu)
        .bind(body.danisan_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn assign_consultant(
        &self,
        customer_id: Uuid,
        danisan_id: Uuid,
    ) -> Result<Customer, sqlx::Error> {
        sqlx::query_as::<_, Customer>(
            "UPDATE customers SET danisan_id = $1 WHERE id = $2 RETURNING *",
        )
        .bind(danisan_id)
        .bind(customer_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn create_customer_note(
        &self,
        customer_id: Uuid,
        body: &CreateCustomerNote,
    ) -> Result<CustomerNote, sqlx::Error> {
        sqlx::query_as::<_, CustomerNote>(
            "INSERT INTO customer_notes (note, created_by, customer_id)
         VALUES ($1, $2, $3)
         RETURNING *",
        )
        .bind(&body.note)
        .bind(body.created_by)
        .bind(customer_id)
        .fetch_one(&self.pool)
        .await
    }
}
