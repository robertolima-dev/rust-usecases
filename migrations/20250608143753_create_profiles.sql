-- Add migration script here
CREATE TABLE profiles (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    bio TEXT,
    birth_date DATE,
    phone TEXT,
    document TEXT,
    profession TEXT,
    avatar TEXT,
    confirm_email BOOLEAN DEFAULT FALSE,
    unsubscribe BOOLEAN DEFAULT FALSE,
    access_level TEXT DEFAULT 'user',
    dt_updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    dt_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
