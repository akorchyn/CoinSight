-- Insert the currency
INSERT INTO currencies (name, symbol) VALUES ('United States Dollar', 'USD');

-- Insert the source
INSERT INTO sources (name) VALUES ('ChainLink');

-- Insert the cryptocurrencies
INSERT INTO cryptocurrencies (name, symbol, description, is_top) 
VALUES 
('Bitcoin', 'BTC', 'Bitcoin is a decentralized digital currency without a central bank or single administrator.', true),
('Ethereum', 'ETH', 'Ethereum is a decentralized, open-source blockchain with smart contract functionality.', true),
('Ripple', 'XRP', 'Ripple is a real-time gross settlement system, currency exchange and remittance network.', true);

-- Insert the mappings (assuming that the cryptocurrencies and sources were inserted in the same order as above)
INSERT INTO source_crypto_mappings (crypto_id, source_id, source_key) 
VALUES 
(1, 1, 'aggregator.btc-usd.data.eth'),
(2, 1, 'aggregator.eth-usd.data.eth'),
(3, 1, 'aggregator.xrp-usd.data.eth');

