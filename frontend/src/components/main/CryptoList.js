// src/components/CryptoList.js

import React from 'react';
import { Link } from 'react-router-dom';
import CryptoCurrencyCard from '../common/CryptoCurrencyCard';
import './CryptoList.css';

const CryptoList = ({ cryptos }) => {
    return (
        <div className="crypto-list">
            {
                Object.keys(cryptos).map((key) => {

                    const asset = cryptos[key];
                    const current_price = asset.aggregatedHistory[0]?.medianPrice || 0;
                    const previousPrice = asset.aggregatedHistory[1]?.medianPrice || 0;

                    return (
                        <Link key={asset.symbol} to={`/asset/${asset.symbol}`} style={{ textDecoration: 'none' }}>
                            <CryptoCurrencyCard
                                assetName={asset.name}
                                symbol={asset.symbol}
                                currentPrice={current_price}
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