-- Add migration script here
CREATE TYPE cephe_enum AS ENUM (
    'KB', 'KD', 'GB', 'GD', 'KB-KD', 'KD-KB'
);

CREATE TABLE properties (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    daire_no    TEXT NOT NULL UNIQUE,
    blok        TEXT NOT NULL,
    kat         TEXT NOT NULL,
    kapi_no     INTEGER NOT NULL,
    daire_tipi  TEXT NOT NULL,
    oda_sayisi  TEXT NOT NULL,
    brut_m2     NUMERIC(8, 2) NOT NULL,
    net_m2      NUMERIC(8, 2) NOT NULL,
    balkon_m2   NUMERIC(8, 2),
    cephe       cephe_enum,
    kiraci_var_mi BOOLEAN NOT NULL DEFAULT false,
    sahip_id    UUID REFERENCES customers(id) ON DELETE SET NULL
);