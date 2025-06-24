-- Add migration script here
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    dt_created TIMESTAMP NOT NULL DEFAULT now(),
    dt_updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    dt_deleted TIMESTAMP
);