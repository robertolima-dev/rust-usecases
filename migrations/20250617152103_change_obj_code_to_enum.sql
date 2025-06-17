-- Alterar a coluna obj_code de TEXT para ENUM obj_code_type
ALTER TABLE notifications
ALTER COLUMN obj_code TYPE obj_code_type
USING obj_code::obj_code_type;