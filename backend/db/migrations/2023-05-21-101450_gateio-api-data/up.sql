WITH rows AS (
    INSERT INTO sources (name) VALUES ('GateIO') RETURNING id
)

INSERT INTO source_crypto_mappings (crypto_id, source_id, source_key) 
VALUES 
(1, (SELECT id FROM rows), 'BTC'),
(2, (SELECT id FROM rows), 'ETH'),
(3, (SELECT id FROM rows), 'XRP');