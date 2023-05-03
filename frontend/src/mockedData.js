// src/mockedData.js


export const mockedCryptoDetails = {
    "0": {
        id: 0,
        name: 'Bitcoin',
        current_price: 45000,
        symbol: 'BTC',
        price_history: [
            { date: '2023-04-01', price: 40000 },
            { date: '2023-04-08', price: 41000 },
            { date: '2023-04-15', price: 42000 },
            { date: '2023-04-22', price: 45000 },
        ],
    },
    "1": {
        id: 1,
        name: 'Ethereum',
        symbol: 'ETH',
        current_price: 3000,
        price_history: [
            { date: '2023-04-01', price: 2800 },
            { date: '2023-04-08', price: 2900 },
            { date: '2023-04-15', price: 3100 },
            { date: '2023-04-22', price: 3000 },
        ],
    },
    "2": {
        name: 'Ripple',
        symbol: 'XRP',
        current_price: 0.9,
        price_history: [
            { date: '2023-04-01', price: 0.8 },
            { date: '2023-04-08', price: 0.82 },
            { date: '2023-04-15', price: 0.85 },
            { date: '2023-04-22', price: 0.9 },
        ],
    },
};
