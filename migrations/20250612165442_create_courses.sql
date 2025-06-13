-- Add migration script here
-- migrations/<timestamp>_create_courses.sql

CREATE TABLE courses (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    price DOUBLE PRECISION NOT NULL,
    month_duration INTEGER NOT NULL,
    author_id UUID NOT NULL REFERENCES users(id),
    dt_start DATE NOT NULL,
    dt_created TIMESTAMP NOT NULL DEFAULT NOW(),
    dt_updated TIMESTAMP NOT NULL DEFAULT NOW(),
    dt_deleted TIMESTAMP
);
