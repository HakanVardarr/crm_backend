use crate::database::Database;
use calamine::{Data, Reader, Xlsx, open_workbook};

pub async fn run(db: &Database, filepath: &str) {
    let mut workbook: Xlsx<_> = open_workbook(filepath).expect("Dosya açılamadı");
    let sheet = workbook.worksheet_range_at(0).unwrap().unwrap();

    let mut created = 0;
    let mut skipped = 0;

    for row in sheet.rows().skip(1) {
        let get = |i: usize| -> String {
            row.get(i)
                .map(|v| match v {
                    Data::String(s) => s.trim().to_string(),
                    Data::Float(f) => f.to_string(),
                    Data::Int(i) => i.to_string(),
                    _ => String::new(),
                })
                .unwrap_or_default()
        };

        let daire_no = get(0);
        if daire_no.is_empty() {
            continue;
        }

        let daire_no = {
            let digits: String = daire_no.chars().filter(|c| c.is_ascii_digit()).collect();
            let letters: String = daire_no.chars().filter(|c| !c.is_ascii_digit()).collect();
            if digits.len() >= 3 {
                let blok_no = &digits[..digits.len() - 3].trim_start_matches('0');
                let kapi_no = digits[digits.len() - 3..].trim_start_matches('0');
                let blok_no = if blok_no.is_empty() { "0" } else { blok_no };
                let kapi_no = if kapi_no.is_empty() { "0" } else { kapi_no };
                format!("{}{}-{}", letters, blok_no, kapi_no)
            } else {
                daire_no
            }
        };

        let property = match db.get_property_by_daire_no(&daire_no).await {
            Ok(Some(p)) => p,
            Ok(None) => {
                eprintln!("{} bulunamadı, atlanıyor.", daire_no);
                skipped += 1;
                continue;
            }
            Err(e) => {
                eprintln!("{} sorgulanırken hata: {}", daire_no, e);
                skipped += 1;
                continue;
            }
        };

        let sahip_adi = get(7);
        if !sahip_adi.is_empty() {
            let customer = db
                .get_or_create_customer(
                    &sahip_adi,
                    &get(8),  // gsm
                    &get(9),  // telefon
                    &get(10), // email
                    &get(11), // acil_kisi
                    &get(12), // uyruk
                )
                .await;

            match customer {
                Ok(c) => {
                    if let Err(e) = db.set_property_owner(property.id, c.id).await {
                        eprintln!("{} sahibi atanırken hata: {}", daire_no, e);
                    }
                }
                Err(e) => eprintln!("{} müşteri oluşturulurken hata: {}", daire_no, e),
            }
        }

        let kiraci_adi = get(13);
        if !kiraci_adi.is_empty()
            && let Err(e) = db.set_kiraci_var_mi(property.id, true).await
        {
            eprintln!("{} kiraci_var_mi güncellenirken hata: {}", daire_no, e);
        }

        created += 1;
    }

    println!("{} kayıt işlendi, {} atlandı.", created, skipped);
}
