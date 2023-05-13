// src/mockedData.js

export const mockedCryptoDetails = {
  'BTC': {
    id: 0,
    name: 'Bitcoin',
    symbol: 'BTC',
    current_price: 45000,
    description: 'Bitcoin is a decentralized digital currency without a central bank or single administrator.',
    market_cap: 840000000000,
    median_value: 42000,
    quartile25: 41000,
    quartile75: 45000,
    price_history: [
      { date: '2023-05-01', price: 42000 },
      { date: '2023-05-02', price: 43000 },
      { date: '2023-05-03', price: 45000 },
      { date: '2023-05-04', price: 46000 },
    ],
  },

  'ETH': {
    id: 1,
    name: 'Ethereum',
    symbol: 'ETH',
    current_price: 3000,
    description: 'Ethereum is a decentralized, open-source blockchain with smart contract functionality.',
    market_cap: 350000000000,
    median_value: 2800,
    quartile25: 2500,
    quartile75: 2900,
    price_history: [
      { date: '2023-05-01', price: 2800 },
      { date: '2023-05-02', price: 2900 },
      { date: '2023-05-03', price: 3000 },
      { date: '2023-05-04', price: 3200 },
    ],
  },

  'XRP': {
    id: 2,
    name: 'Ripple',
    symbol: 'XRP',
    current_price: 1.5,
    description: 'Ripple is a real-time gross settlement system, currency exchange and remittance network.',
    market_cap: 70000000000,
    median_value: 1.3,
    quartile25: 1.2,
    quartile75: 1.4,
    price_history: [
      { date: '2023-05-01', price: 1.2 },
      { date: '2023-05-02', price: 1.3 },
      { date: '2023-05-03', price: 1.5 },
      { date: '2023-05-04', price: 1.7 },
    ],
  },
};
