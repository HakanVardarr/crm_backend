use crate::database::Database;
use crate::models::{CreateProperty, Property, PropertyWithCustomer};

use uuid::Uuid;

impl Database {
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

    pub async fn list_properties(&self) -> Result<Vec<PropertyWithCustomer>, sqlx::Error> {
        let data = sqlx::query_as::<_, PropertyWithCustomer>(
            "SELECT
            p.id, p.daire_no, p.blok, p.kat, p.kapi_no, p.daire_tipi,
            p.oda_sayisi, p.brut_m2, p.net_m2, p.balkon_m2, p.cephe,
            p.kiraci_var_mi, p.sahip_id,
            c.ad_soyad AS sahip_ad_soyad
         FROM properties p
         LEFT JOIN customers c ON c.id = p.sahip_id
         ORDER BY p.daire_no",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(data)
    }
}
