use crate::database::Database;
use crate::models::CreateProperty;
use bigdecimal::BigDecimal;
use calamine::{Data, Reader, Xlsx, open_workbook};
use std::str::FromStr;

pub async fn run(db: &Database, filepath: &str) {
    let mut workbook: Xlsx<_> = open_workbook(filepath).expect("Dosya açılamadı");
    let sheet = workbook.worksheet_range_at(0).unwrap().unwrap();

    let mut created = 0;
    let mut updated = 0;
    let mut errors = 0;

    for row in sheet.rows() {
        let get_str = |i: usize| -> Option<String> {
            row.get(i).and_then(|v| match v {
                Data::String(s) => Some(s.trim().to_string()),
                Data::Float(f) => Some(f.to_string()),
                Data::Int(i) => Some(i.to_string()),
                _ => None,
            })
        };

        let get_decimal = |i: usize| -> Option<BigDecimal> {
            row.get(i).and_then(|v| match v {
                Data::Float(f) => BigDecimal::from_str(&f.to_string()).ok(),
                Data::Int(i) => BigDecimal::from_str(&i.to_string()).ok(),
                _ => None,
            })
        };

        let blok = match get_str(1) {
            Some(b) => b,
            None => continue,
        };

        let kapi_no: i32 = match get_str(3).and_then(|v| v.parse().ok()) {
            Some(k) => k,
            None => continue,
        };

        let daire_no = format!("{}{}", blok, kapi_no);

        let prop = CreateProperty {
            daire_no,
            blok: blok.trim_end_matches('-').to_string(),
            kat: get_str(2).unwrap_or_default(),
            kapi_no,
            oda_sayisi: get_str(4).unwrap_or_default(),
            daire_tipi: get_str(5).unwrap_or_default(),
            brut_m2: get_decimal(6).unwrap_or_default(),
            net_m2: get_decimal(7).unwrap_or_default(),
            balkon_m2: get_decimal(8),
            cephe: None, // parse edebilirsin
            kiraci_var_mi: false,
            sahip_id: None,
        };

        match db.upsert_property(&prop).await {
            Ok(is_new) => {
                if is_new {
                    created += 1;
                } else {
                    updated += 1;
                }
            }
            Err(e) => {
                eprintln!("Hata — daire_no {}: {}", prop.daire_no, e);
                errors += 1;
            }
        }
    }

    println!(
        "{} oluşturuldu, {} güncellendi, {} hata",
        created, updated, errors
    );
}
