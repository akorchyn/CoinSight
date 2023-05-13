-- Create table for cryptocurrencies
CREATE TABLE cryptocurrencies (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  symbol VARCHAR(50) NOT NULL,
  description TEXT NOT NULL
);

-- Add unique constraint on the lower case symbol in cryptocurrencies
CREATE UNIQUE INDEX unique_lower_cryptocurrency_symbol ON cryptocurrencies (LOWER(symbol));

-- Create table for sources
CREATE TABLE sources (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL
);

-- Create table for currencies
CREATE TABLE currencies (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  symbol VARCHAR(50) NOT NULL
);

-- Add unique constraint on the lower case symbol in currencies
CREATE UNIQUE INDEX unique_lower_currency_symbol ON currencies (LOWER(symbol));

-- Create table for source-cryptocurrency mappings
CREATE TABLE source_crypto_mappings (
  id SERIAL PRIMARY KEY,
  crypto_id INT REFERENCES cryptocurrencies(id) NOT NULL,
  source_id INT REFERENCES sources(id) NOT NULL,
  source_key VARCHAR(255) NOT NULL
);

-- Create table for prices
CREATE TABLE prices (
  id SERIAL PRIMARY KEY,
  crypto_id INT REFERENCES cryptocurrencies(id) NOT NULL,
  source_id INT REFERENCES sources(id) NOT NULL,
  currency_id INT REFERENCES currencies(id) NOT NULL,
  price DECIMAL(20, 8) NOT NULL,
  timestamp TIMESTAMP NOT NULL
);

-- Create table for aggregated prices
CREATE TABLE aggregated_prices (
  id SERIAL PRIMARY KEY,
  crypto_id INT REFERENCES cryptocurrencies(id) NOT NULL,
  currency_id INT REFERENCES currencies(id) NOT NULL,
  median_price DECIMAL(20, 8) NOT NULL,
  first_quartile_price DECIMAL(20, 8) NOT NULL,
  third_quartile_price DECIMAL(20, 8) NOT NULL,
  timestamp TIMESTAMP NOT NULL
);

-- Index for faster search in prices
CREATE INDEX idx_prices_crypto ON prices(crypto_id);
CREATE INDEX idx_prices_source ON prices(source_id);
CREATE INDEX idx_prices_currency ON prices(currency_id);
CREATE INDEX idx_prices_timestamp ON prices(timestamp);

-- Index for faster search in aggregated prices
CREATE INDEX idx_aggprices_crypto ON aggregated_prices(crypto_id);
CREATE INDEX idx_aggprices_currency ON aggregated_prices(currency_id);
CREATE INDEX idx_aggprices_timestamp ON aggregated_prices(timestamp);

COMMENT ON TABLE cryptocurrencies IS 'Table to store basic information about each cryptocurrency.';

COMMENT ON TABLE sources IS 'Table to store different sources of cryptocurrency data.';

COMMENT ON TABLE currencies IS 'Table to store different fiat currencies that the prices of cryptocurrencies can be represented in.';

COMMENT ON TABLE source_crypto_mappings IS 'Table to map each cryptocurrency to its unique identifier on each source.';

COMMENT ON TABLE prices IS 'Table to store the price of each cryptocurrency in each currency from each source at different points in time.';

COMMENT ON TABLE aggregated_prices IS 'Table to store the aggregated prices for each cryptocurrency in each currency, including the median, 25% quartile, and 75% quartile prices.';
