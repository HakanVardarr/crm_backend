use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Serialize, sqlx::FromRow)]
pub struct PropertyWithCustomer {
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
    pub sahip_ad_soyad: Option<String>,
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
