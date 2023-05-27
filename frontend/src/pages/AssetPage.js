// src/components/CryptoDetails.js

import { useParams } from 'react-router-dom';
import React from 'react';
import './css/AssetPage.css';
import AssetInfo from '../components/asset/StatisticInfo';
import { gql, useQuery } from '@apollo/client';
import Loading from '../components/common/Loading';
import CryptoChart from '../components/asset/CryptoChart';


const GRAPHQL_REQUEST = gql`
query getCrypto($symbol: String!) {
  crypto {
    bySymbol(symbol: $symbol) {
      id
      symbol
      name
      description
      latestAggregatedPrice {
        id
        medianPrice
        firstQuartilePrice
        thirdQuartilePrice
        timestamp
      }
    }
  }
}
`;

const AssetPage = () => {
  const { symbol } = useParams();
  const { loading, error, data } = useQuery(GRAPHQL_REQUEST, {
    variables: { symbol },
    pollInterval: 5000,
  });


  if (loading) {
    return <Loading />;
  }

  if (error) {
    return <p>Error: {error.message}</p>;
  }

  const cryptocurrency = data?.crypto.bySymbol;

  const priceInfo = cryptocurrency.latestAggregatedPrice;
  return (
    <div className="container" >
      <div className="details">
        <AssetInfo assetData={{ ...cryptocurrency, ...priceInfo }} />
        <CryptoChart crypto_id={cryptocurrency.id} />
      </div>
    </div >
  );
};

export default AssetPage;
