-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(16) UNIQUE,
    email VARCHAR(256) UNIQUE,
    password_hash VARCHAR(256),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);
INSERT INTO users (username, created_at, updated_at)
VALUES (
        'a',
        '2020-12-20',
        '2020-12-20'
    ),
    (
        'b',
        '2020-12-20',
        '2020-12-20'
    ),
    (
        'c',
        '2020-12-20',
        '2020-12-20'
    ),
    (
        'd',
        '2020-12-20',
        '2020-12-20'
    );
