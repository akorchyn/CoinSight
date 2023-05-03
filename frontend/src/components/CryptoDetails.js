// src/components/CryptoDetails.js

import React, { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { Line } from 'react-chartjs-2';
import { Chart } from 'chart.js';
import zoomPlugin from 'chartjs-plugin-zoom';
import { registerables, CategoryScale, LinearScale } from 'chart.js';
import 'chartjs-adapter-date-fns';
import './CryptoDetails.css';

// Register required scales and plugins
Chart.register(...registerables, CategoryScale, LinearScale, zoomPlugin);

const CryptoDetails = ({ cryptos }) => {
    const { id } = useParams();
    const [crypto, setCrypto] = useState(null);
    const [chartData, setChartData] = useState(null);

    useEffect(() => {
        const fetchData = async () => {
            const cryptoData = cryptos[id];

            if (cryptoData) {
                setCrypto(cryptoData);

                // Format the data for the chart
                const chartData = {
                    labels: cryptoData.price_history.map(item => item.date),
                    datasets: [
                        {
                            label: 'Price History',
                            data: cryptoData.price_history.map(item => item.price),
                            fill: false,
                            backgroundColor: 'rgba(75, 192, 192, 0.6)',
                            borderColor: 'rgba(75, 192, 192, 1)',
                        },
                    ],
                };

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
                <Line data={chartData} className="chart" />
            </div>
        </div>
    );
};

export default CryptoDetails;
