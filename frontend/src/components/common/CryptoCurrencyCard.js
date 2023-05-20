import React from 'react';
import { Card } from 'antd';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faArrowTrendUp, faArrowTrendDown } from '@fortawesome/free-solid-svg-icons';
import './CryptoCurrencyCard.css';
import Price from './Price';

const CryptoCurrencyCard = ({ assetName, symbol, currentPrice, previousPrice }) => {

    return (
        <Card className="crypto-card" hoverable type="inner">
            <div className="crypto-content">
                <div className="crypto-icon-container">
                </div>
                <div className="crypto-name">
                    <p>{assetName} <span className="symbol-text">{symbol}</span></p>
                </div>

                <Price price={currentPrice} float_point={3} isPriceUp={currentPrice != previousPrice ? currentPrice > previousPrice : undefined} />
            </div>
        </Card >
    );
};

export default CryptoCurrencyCard;
