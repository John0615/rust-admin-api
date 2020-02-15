-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    salt VARCHAR(255) NOT NULL,
    password_digest VARCHAR(255) NOT NULL,
    phone VARCHAR(20) NOT NULL UNIQUE,
    email VARCHAR(120) NOT NULL UNIQUE,
    role VARCHAR(32) NOT NULL DEFAULT 'user',
    login_name VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP
);

CREATE UNIQUE INDEX users__email_idx ON users(email);
