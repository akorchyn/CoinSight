import React from 'react';
import { Card } from 'antd';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faArrowTrendUp, faArrowTrendDown } from '@fortawesome/free-solid-svg-icons';
import Icon from "react-crypto-icons";
import './CryptoCurrencyCard.css';

const CryptoCurrencyCard = ({ assetName, symbol, currentPrice, previousPrice }) => {
    const isPriceUp = currentPrice > previousPrice;
    const priceColor = isPriceUp ? 'text-success' : 'text-danger';
    const arrowIcon = isPriceUp ? faArrowTrendUp : faArrowTrendDown;

    return (
        <Card className="crypto-card" hoverable type="inner">
            <div className="crypto-content">
                <div className="crypto-icon-container">
                    <Icon name={symbol.toLowerCase()} alt={`${assetName} icon`} size={50} className="crypto-icon" />
                </div>
                <div className="crypto-name">
                    <p>{assetName} <span className="symbol-text">{symbol}</span></p>
                </div>

                <div className="crypto-price">
                    <p className={priceColor}>
                        <span>${currentPrice.toFixed(2)}</span>
                        <FontAwesomeIcon icon={arrowIcon} />
                    </p>
                </div>
            </div>
        </Card >
    );
};

export default CryptoCurrencyCard;