// src/components/CryptoList.js

import React from 'react';
import { Link } from 'react-router-dom';
import styles from './CryptoList.module.css';

const CryptoList = ({ cryptos }) => {
    return (
        <div className={styles['crypto-list']}>
            <table>
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Symbol</th>
                        <th>Price</th>
                    </tr>
                </thead>
                <tbody>
                    {Object.keys(cryptos).map((key) => {
                        const crypto = cryptos[key];
                        console.log(crypto);

                        const previousPrice = crypto.price_history[crypto.price_history.length - 2]?.price || 0;
                        const priceChange = ((crypto.current_price - previousPrice) / previousPrice) * 100;

                        const priceClass = priceChange > 0 ? styles.green : styles.red;

                        return (
                            <tr key={key}>
                                <td>
                                    <Link to={`/details/${key}`} className={styles['crypto-name']}>
                                        {crypto.name}
                                    </Link>
                                </td>
                                <td>{crypto.symbol.toUpperCase()}</td>
                                <td className={priceClass}>${crypto.current_price.toFixed(2)}</td>
                            </tr>
                        );
                    })}
                </tbody>
            </table>
        </div>
    );
};

export default CryptoList;