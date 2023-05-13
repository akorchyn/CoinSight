-- Delete the aggregated prices
DELETE FROM aggregated_prices WHERE crypto_id IN (1, 2, 3) AND currency_id = 1;

-- Delete the prices
DELETE FROM prices WHERE crypto_id IN (1, 2, 3) AND source_id = 1 AND currency_id = 1;

-- Delete the mappings
DELETE FROM source_crypto_mappings WHERE crypto_id IN (1, 2, 3) AND source_id = 1;

-- Delete the cryptocurrencies
DELETE FROM cryptocurrencies WHERE id IN (1, 2, 3);

-- Delete the source
DELETE FROM sources WHERE name = 'Coin Market Cap';

-- Delete the currency
DELETE FROM currencies WHERE name = 'United States Dollar';
