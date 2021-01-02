-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(16) UNIQUE NOT NULL,
    email VARCHAR(256) UNIQUE,
    password_hash VARCHAR(256),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);
