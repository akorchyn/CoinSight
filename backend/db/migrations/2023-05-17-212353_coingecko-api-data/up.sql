WITH rows AS (
    INSERT INTO sources (name) VALUES ('CoinGecko') RETURNING id
)

INSERT INTO source_crypto_mappings (crypto_id, source_id, source_key) 
VALUES 
(1, (SELECT id FROM rows), 'bitcoin'),
(2, (SELECT id FROM rows), 'ethereum'),
(3, (SELECT id FROM rows), 'ripple');