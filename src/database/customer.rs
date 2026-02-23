use crate::database::Database;
use crate::models::{CreateCustomer, CreateCustomerNote, Customer, CustomerDetail, CustomerNote};
use uuid::Uuid;

impl Database {
    pub async fn list_customers(&self) -> Result<Vec<Customer>, sqlx::Error> {
        let data = sqlx::query_as::<_, Customer>(
        "SELECT id, ad_soyad, gsm, telefon, email, acil_kisi, uyruk, en_son_gorusuldu, danisan_id
         FROM customers ORDER BY ad_soyad"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(data)
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
}
