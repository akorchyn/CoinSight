// src/components/CryptoDetails.js

import { useParams } from 'react-router-dom';
import './css/AssetPage.css';
import PriceHistoryChart from '../components/asset/Charts';
import AssetInfo from '../components/asset/StatisticInfo';
import { gql, useQuery } from '@apollo/client';

const GRAPHQL_REQUEST = gql`
  query getCrypto($symbol: String!) {
    cryptocurrency(symbol: $symbol) {
      symbol,
      name,
      description,
      latestAggregatedPrice {
        medianPrice,
        firstQuartilePrice,
        thirdQuartilePrice,
        timestamp
      },
      aggregatedHistory {
        medianPrice, firstQuartilePrice, thirdQuartilePrice, timestamp
      }
      history(sourceId: 1) {
        price,  timestamp
      }
    },
  }
`;

const AssetPage = () => {
  const { symbol } = useParams();
  const { loading, error, data } = useQuery(GRAPHQL_REQUEST, {
    variables: { symbol }
  });

  if (loading) {
    return null;
  }

  if (error) {
    return <p>Error: {error.message}</p>;
  }

  const cryptocurrency = data?.cryptocurrency;

  const priceInfo = cryptocurrency.latestAggregatedPrice;
  const history = cryptocurrency.history.slice().reverse();
  return (
    <div className="container">
      <h2 className="title">{cryptocurrency.name} Details</h2>
      <div className="details">
        <AssetInfo assetData={{ ...cryptocurrency, ...priceInfo }} />
        <PriceHistoryChart data={history} />
      </div>
    </div>
  );
};

export default AssetPage;
