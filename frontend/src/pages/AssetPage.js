// src/components/CryptoDetails.js

import React, { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import './css/AssetPage.css';
import PriceHistoryChart from '../components/asset/Charts';
import AssetInfo from '../components/asset/StatisticInfo';
import { gql, useQuery } from '@apollo/client';

const GET_CRYPTO_CURRENCY = gql`
  query getCrypto($symbol: String!) {
    cryptocurrency(symbol: $symbol) {
      id
      name
      description
    }
  }
`;

const GET_CRYPTO_DETAILS = gql`
  query GetCryptoDetails($cryptoId: Int!) {
    aggregatedPriceLatest(cryptoId: $cryptoId, currencyId: 1) {
        medianPrice,
        firstQuartilePrice, thirdQuartilePrice
    }
  }
`;

const AssetPage = () => {
    const { symbol } = useParams();
    const { loading, error, data } = useQuery(GET_CRYPTO_CURRENCY, {
        variables: { symbol }
    });

    const cryptocurrency = data?.cryptocurrency;
    const id = cryptocurrency?.id;

    const { loading1, error1, data: aggregatedPrice } = useQuery(GET_CRYPTO_DETAILS, {
        enable: !!id,
        variables: { cryptoId: id }
    });

    if (loading || loading1 || id === undefined) {
        return null;
    }

    if (error || error1) {
        return <p>Error: {error.message || error1.message}</p>;
    }

    const priceInfo = aggregatedPrice?.aggregatedPriceLatest;



    return (
        <div className="container">
            <h2 className="title">{cryptocurrency.name} Details</h2>
            <div className="details">
                <AssetInfo assetData={{ ...cryptocurrency, ...priceInfo }} />
                {/* <PriceHistoryChart data={chartData} /> */}
            </div>
        </div>
    );
};

export default AssetPage;
