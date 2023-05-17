// src/components/CryptoDetails.js

import { useParams } from 'react-router-dom';
import './css/AssetPage.css';
import AssetInfo from '../components/asset/StatisticInfo';
import { gql, useQuery } from '@apollo/client';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from "recharts";


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
    },
  }
`;

const AssetPage = () => {
  const formatXAxis = (tickItem) => {
    // Format tick labels as dates using a library like moment.js
    return tickItem;
  };

  const formatTooltip = (value, name, props) => {
    // Format tooltip values using a library like numeral.js
    return [`${name}: ${value}`];
  };



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
  const history = cryptocurrency.aggregatedHistory.slice().reverse();
  return (
    <div className="container">
      <h2 className="title">{cryptocurrency.name} Details</h2>
      <div className="details">
        <AssetInfo assetData={{ ...cryptocurrency, ...priceInfo }} />
        <ResponsiveContainer width="100%" height={500}>
          <LineChart data={history}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="timestamp" tickFormatter={formatXAxis} />
            <YAxis />
            <Tooltip formatter={formatTooltip} />
            <Line
              type="monotone"
              dataKey="medianPrice"
              stroke={"#8884d8"}
              strokeWidth={2}
              dot={true}
            />
            <Line
              type="monotone"
              dataKey="firstQuartilePrice"
              stroke={"#8884d8"}
              strokeWidth={2}
              dot={true}
            />
            <Line
              type="monotone"
              dataKey="thirdQuartilePrice"
              stroke={"#8884d8"}
              strokeWidth={2}
              dot={true}
            />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
};

export default AssetPage;
