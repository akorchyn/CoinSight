-- Insert the currency
INSERT INTO currencies (name, symbol) VALUES ('United States Dollar', 'USD');

-- Insert the source
INSERT INTO sources (name) VALUES ('Coin Market Cap');

-- Insert the cryptocurrencies
INSERT INTO cryptocurrencies (name, symbol, description) 
VALUES 
('Bitcoin', 'BTC', 'Bitcoin is a decentralized digital currency without a central bank or single administrator.'),
('Ethereum', 'ETH', 'Ethereum is a decentralized, open-source blockchain with smart contract functionality.'),
('Ripple', 'XRP', 'Ripple is a real-time gross settlement system, currency exchange and remittance network.');

-- Insert the mappings (assuming that the cryptocurrencies and sources were inserted in the same order as above)
INSERT INTO source_crypto_mappings (crypto_id, source_id, source_key) 
VALUES 
(1, 1, 'BTC'),
(2, 1, 'ETH'),
(3, 1, 'XRP');

-- Insert the prices (assuming that the cryptocurrencies and sources were inserted in the same order as above)
INSERT INTO prices (crypto_id, source_id, currency_id, price, timestamp) 
VALUES 
(1, 1, 1, 42000, '2023-05-01'),
(1, 1, 1, 43000, '2023-05-02'),
(1, 1, 1, 45000, '2023-05-03'),
(1, 1, 1, 46000, '2023-05-04'),
(2, 1, 1, 2800, '2023-05-01'),
(2, 1, 1, 2900, '2023-05-02'),
(2, 1, 1, 3000, '2023-05-03'),
(2, 1, 1, 3200, '2023-05-04'),
(3, 1, 1, 1.2, '2023-05-01'),
(3, 1, 1, 1.3, '2023-05-02'),
(3, 1, 1, 1.5, '2023-05-03'),
(3, 1, 1, 1.7, '2023-05-04');

-- Insert the aggregated prices (assuming that the cryptocurrencies were inserted in the same order as above)
INSERT INTO aggregated_prices (crypto_id, currency_id, median_price, first_quartile_price, third_quartile_price, timestamp) 
VALUES 
(1, 1, 42000, 41000, 45000, '2023-05-04'),
(2, 1, 2800, 2500, 2900, '2023-05-04'),
(3, 1, 1.3, 1.2, 1.4, '2023-05-04');
