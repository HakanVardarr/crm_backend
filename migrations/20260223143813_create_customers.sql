-- Add migration script here
CREATE TABLE customers (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ad_soyad    TEXT NOT NULL,
    gsm         TEXT,
    telefon     TEXT,
    email       TEXT,
    acil_kisi   TEXT,
    uyruk       TEXT,
    en_son_gorusuldu TIMESTAMPTZ,
    danisan_id  UUID REFERENCES users(id) ON DELETE SET NULL
);