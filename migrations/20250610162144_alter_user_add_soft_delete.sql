-- Add migration script here
-- Adiciona os campos novos
ALTER TABLE users
ADD COLUMN dt_deleted TIMESTAMP DEFAULT NULL;
