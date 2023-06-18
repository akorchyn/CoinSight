CREATE TABLE telegram_auth (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) NOT NULL UNIQUE,
    telegram_id BIGINT,
    auth_code VARCHAR(255) NOT NULL
);