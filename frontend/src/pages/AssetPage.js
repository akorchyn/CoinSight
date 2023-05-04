// src/components/CryptoDetails.js

import React, { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import './css/AssetPage.css';
import PriceHistoryChart from '../components/asset/Charts';
import AssetInfo from '../components/asset/StatisticInfo';


const AssetPage = ({ cryptos }) => {
    const { id } = useParams();
    const [crypto, setCrypto] = useState(null);
    const [chartData, setChartData] = useState(null);

    useEffect(() => {
        const fetchData = async () => {
            const cryptoData = cryptos[id];
            if (cryptoData) {
                setCrypto(cryptoData);

                // Format the data for the chart
                const chartData = cryptoData.price_history;

                setChartData(chartData);
            }
        };

        fetchData();
    }, [id, cryptos]);

    if (!crypto) {
        return <div>Loading...</div>;
    }

    return (
        <div className="container">
            <h2 className="title">{crypto.name} Details</h2>
            <div className="details">
                <p>Current Price: ${crypto.current_price}</p>
                <AssetInfo assetData={crypto} />
                <PriceHistoryChart data={chartData} />
            </div>
        </div>
    );
};

export default AssetPage;
