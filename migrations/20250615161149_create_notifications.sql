-- Add migration script here
CREATE TYPE obj_code_type AS ENUM ('platform', 'user');

CREATE TABLE notifications (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    obj_code obj_code_type NOT NULL,
    obj_id UUID,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);