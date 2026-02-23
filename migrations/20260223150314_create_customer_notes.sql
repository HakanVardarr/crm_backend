-- Add migration script here
CREATE TABLE customer_notes (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    note        TEXT NOT NULL,
    created_by  UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);