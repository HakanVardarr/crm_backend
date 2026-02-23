use crate::models::{
    CreateCustomer, CreateCustomerNote, CreateProperty, CreateUser, Customer, CustomerDetail,
    CustomerNote, PaginatedCustomers, PaginatedProperties, Property, UserWithPassword,
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

    pub async fn list_customers(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<PaginatedCustomers, sqlx::Error> {
        let offset = (page - 1) * page_size;

        let total_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM customers")
            .fetch_one(&self.pool)
            .await?;

        let data = sqlx::query_as::<_, Customer>(
        "SELECT id, ad_soyad, gsm, telefon, email, acil_kisi, uyruk, en_son_gorusuldu, danisan_id
         FROM customers ORDER BY ad_soyad LIMIT $1 OFFSET $2"
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(PaginatedCustomers {
            data,
            total_count,
            page,
            total_pages: (total_count + page_size - 1) / page_size,
        })
    }

    pub async fn customer_detail(&self, customer_id: Uuid) -> Result<CustomerDetail, sqlx::Error> {
        let customer: Customer = sqlx::query_as::<_, Customer>(
        "SELECT id, ad_soyad, gsm, telefon, email, acil_kisi, uyruk, en_son_gorusuldu, danisan_id 
         FROM customers WHERE id = $1"
        )
        .bind(customer_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

        let notes: Vec<CustomerNote> = sqlx::query_as::<_, CustomerNote>(
            "SELECT id, note, created_by, customer_id, created_at 
         FROM customer_notes WHERE customer_id = $1",
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(CustomerDetail {
            customer_info: customer,
            customer_notes: notes,
        })
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

    pub async fn upsert_property(&self, prop: &CreateProperty) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
        "INSERT INTO properties (daire_no, blok, kat, kapi_no, oda_sayisi, daire_tipi, brut_m2, net_m2, balkon_m2, cephe, kiraci_var_mi)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
         ON CONFLICT (daire_no) DO UPDATE SET
             blok = EXCLUDED.blok,
             kat = EXCLUDED.kat,
             kapi_no = EXCLUDED.kapi_no,
             oda_sayisi = EXCLUDED.oda_sayisi,
             daire_tipi = EXCLUDED.daire_tipi,
             brut_m2 = EXCLUDED.brut_m2,
             net_m2 = EXCLUDED.net_m2,
             balkon_m2 = EXCLUDED.balkon_m2,
             cephe = EXCLUDED.cephe,
             kiraci_var_mi = EXCLUDED.kiraci_var_mi"
        )
        .bind(&prop.daire_no)
        .bind(&prop.blok)
        .bind(&prop.kat)
        .bind(prop.kapi_no)
        .bind(&prop.oda_sayisi)
        .bind(&prop.daire_tipi)
        .bind(&prop.brut_m2)
        .bind(&prop.net_m2)
        .bind(&prop.balkon_m2)
        .bind(&prop.cephe)
        .bind(prop.kiraci_var_mi)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() == 1)
    }

    pub async fn get_property_by_daire_no(
        &self,
        daire_no: &str,
    ) -> Result<Option<Property>, sqlx::Error> {
        sqlx::query_as::<_, Property>("SELECT * FROM properties WHERE daire_no = $1")
            .bind(daire_no)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_or_create_customer(
        &self,
        ad_soyad: &str,
        gsm: &str,
        telefon: &str,
        email: &str,
        acil_kisi: &str,
        uyruk: &str,
    ) -> Result<Customer, sqlx::Error> {
        sqlx::query_as::<_, Customer>(
            "INSERT INTO customers (ad_soyad, gsm, telefon, email, acil_kisi, uyruk)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *",
        )
        .bind(ad_soyad)
        .bind(if gsm.is_empty() { None } else { Some(gsm) })
        .bind(if telefon.is_empty() {
            None
        } else {
            Some(telefon)
        })
        .bind(if email.is_empty() { None } else { Some(email) })
        .bind(if acil_kisi.is_empty() {
            None
        } else {
            Some(acil_kisi)
        })
        .bind(if uyruk.is_empty() { None } else { Some(uyruk) })
        .fetch_one(&self.pool)
        .await
    }

    pub async fn set_property_owner(
        &self,
        property_id: Uuid,
        customer_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE properties SET sahip_id = $1 WHERE id = $2")
            .bind(customer_id)
            .bind(property_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_kiraci_var_mi(
        &self,
        property_id: Uuid,
        kiraci_var_mi: bool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE properties SET kiraci_var_mi = $1 WHERE id = $2")
            .bind(kiraci_var_mi)
            .bind(property_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn list_properties(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<PaginatedProperties, sqlx::Error> {
        let offset = (page - 1) * page_size;

        let total_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM properties")
            .fetch_one(&self.pool)
            .await?;

        let data = sqlx::query_as::<_, Property>(
            "SELECT * FROM properties ORDER BY daire_no LIMIT $1 OFFSET $2",
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(PaginatedProperties {
            data,
            total_count,
            page,
            total_pages: (total_count + page_size - 1) / page_size,
        })
    }
}
