// src/components/CryptoList.js

import React from 'react';
import { Link } from 'react-router-dom';
import CryptoCurrencyCard from './CryptoCurrencyCard';
import './CryptoList.css';

const CryptoList = ({ cryptos }) => {
    return (
        <div className="crypto-list">
            {
                Object.keys(cryptos).map((key) => {

                    const asset = cryptos[key];
                    const previousPrice = asset.price_history[asset.price_history.length - 2]?.price || 0;

                    return (
                        <Link key={asset.symbol} to={`/details/${key}`} style={{ textDecoration: 'none' }}>
                            <CryptoCurrencyCard
                                assetName={asset.name}
                                symbol={asset.symbol}
                                currentPrice={asset.current_price}
                                previousPrice={previousPrice}
                            />
                        </Link>
                    );
                })
            }
        </div>
    );
};

export default CryptoList;