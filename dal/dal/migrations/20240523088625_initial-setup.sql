-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    username VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    user_role VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    uuid VARCHAR NOT NULL DEFAULT gen_random_uuid() UNIQUE,
    date_created TIMESTAMP NOT NULL DEFAULT NOW(),
    last_logged_in TIMESTAMP NOT NULL DEFAULT NOW(),
    blocked BOOLEAN NOT NULL DEFAULT FALSE
);


CREATE TABLE IF NOT EXISTS role_permissions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR NOT NULL,
    CONSTRAINT unique_user_role UNIQUE (user_id, role)  -- Ensure combination of user_id and role is unique
);


CREATE TABLE IF NOT EXISTS rate_limit_entries (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL,
    rate_limit_period_start TIMESTAMP DEFAULT NOW(),
    count INTEGER DEFAULT 1
);


CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    due_date TIMESTAMP,
    assigned_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    assigned_to INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    description TEXT,
    date_assigned TIMESTAMP NOT NULL DEFAULT NOW(),
    date_finished TIMESTAMP,
    finished BOOLEAN NOT NULL DEFAULT FALSE
);
