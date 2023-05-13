CREATE TABLE currencies (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    symbol VARCHAR(10) NOT NULL
);

CREATE TABLE cryptocurrencies (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    symbol VARCHAR(10) NOT NULL,
    description TEXT
);

CREATE TABLE sources (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

CREATE TABLE prices (
    id SERIAL PRIMARY KEY,
    crypto_id INTEGER REFERENCES cryptocurrencies(id),
    source_id INTEGER REFERENCES sources(id),
    currency_id INTEGER REFERENCES currencies(id),
    price DECIMAL(38,18) NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE aggregated_prices (
    id SERIAL PRIMARY KEY,
    crypto_id INTEGER REFERENCES cryptocurrencies(id),
    currency_id INTEGER REFERENCES currencies(id),
    median_price DECIMAL(38,18) NOT NULL,
    first_quartile_price DECIMAL(38,18) NOT NULL,
    third_quartile_price DECIMAL(38,18) NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
