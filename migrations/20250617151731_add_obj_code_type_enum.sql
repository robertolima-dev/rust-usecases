DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'obj_code_type') THEN
        CREATE TYPE obj_code_type AS ENUM ('platform', 'user');
    END IF;
END$$;

ALTER TABLE notifications
ALTER COLUMN obj_code TYPE obj_code_type
USING obj_code::obj_code_type;