use crate::database::Database;
use crate::models::{
    CreateCustomer, CreateCustomerNote, Customer, CustomerDetail, CustomerNote,
    CustomerWithProperties, Property,
};
use uuid::Uuid;

impl Database {
    pub async fn list_customers(&self) -> Result<Vec<CustomerWithProperties>, sqlx::Error> {
        sqlx::query_as::<_, CustomerWithProperties>(
            "SELECT 
            c.id, c.ad_soyad, c.gsm, c.telefon, c.email, c.acil_kisi, 
            c.uyruk, c.en_son_gorusuldu, c.danisan_id,
            COUNT(p.id) AS property_count,
            STRING_AGG(p.daire_no, ', ' ORDER BY p.daire_no) AS daire_nolar
         FROM customers c
         LEFT JOIN properties p ON p.sahip_id = c.id
         GROUP BY c.id
         ORDER BY c.ad_soyad",
        )
        .fetch_all(&self.pool)
        .await
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

        let properties: Vec<Property> = sqlx::query_as::<_, Property>(
            "SELECT * FROM properties WHERE sahip_id = $1 ORDER BY daire_no",
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(CustomerDetail {
            customer_info: customer,
            customer_notes: notes,
            customer_properties: properties,
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
        user_id: Uuid,
        body: &CreateCustomerNote,
    ) -> Result<CustomerNote, sqlx::Error> {
        sqlx::query_as::<_, CustomerNote>(
            "INSERT INTO customer_notes (note, created_by, customer_id)
         VALUES ($1, $2, $3)
         RETURNING *",
        )
        .bind(&body.note)
        .bind(user_id)
        .bind(customer_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_customer_note(&self, note_id: Uuid) -> Result<(), sqlx::Error> {
        let result = sqlx::query("DELETE FROM customer_notes WHERE id = $1")
            .bind(note_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
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
