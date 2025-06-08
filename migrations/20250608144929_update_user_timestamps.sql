-- Add migration script here
-- Remove o campo antigo
ALTER TABLE users DROP COLUMN created_at;

-- Adiciona os campos novos
ALTER TABLE users
ADD COLUMN dt_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN dt_updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;