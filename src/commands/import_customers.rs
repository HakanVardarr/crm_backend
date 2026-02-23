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
