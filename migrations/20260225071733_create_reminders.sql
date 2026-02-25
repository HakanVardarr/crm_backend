-- Add migration script here
CREATE TABLE reminders (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title         TEXT NOT NULL,
    reminder_date TIMESTAMPTZ NOT NULL,
    created_by    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    customer_id   UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE
);