-- Delete the mappings
DELETE FROM source_crypto_mappings WHERE crypto_id IN (1, 2, 3) AND source_id = 1;

-- Delete the cryptocurrencies
DELETE FROM cryptocurrencies WHERE id IN (1, 2, 3);

-- Delete the source
DELETE FROM sources WHERE name = 'ChainLink';

-- Delete the currency
DELETE FROM currencies WHERE name = 'United States Dollar';
