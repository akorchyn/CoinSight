DROP TABLE aggregated_prices;
DROP INDEX idx_aggprices_crypto;
DROP INDEX idx_aggprices_currency;
DROP INDEX idx_aggprices_timestamp;

DROP TABLE prices;
DROP INDEX idx_prices_crypto;
DROP INDEX idx_prices_source;
DROP INDEX idx_prices_currency;
DROP INDEX idx_prices_timestamp;

DROP TABLE source_crypto_mappings;

DROP TABLE currencies;
DROP INDEX unique_lower_currency_symbol;

DROP TABLE sources;

DROP TABLE cryptocurrencies;
DROP INDEX unique_lower_cryptocurrency_symbol;
