// src/components/CryptoDetails.js

import { useParams } from 'react-router-dom';
import React, { useEffect } from 'react';
import './css/AssetPage.css';
import AssetInfo from '../components/asset/StatisticInfo';
import { gql, useQuery } from '@apollo/client';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from "recharts";
import Loading from '../components/common/Loading';


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
      aggregatedHistory {
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
  const formatXAxis = (tickItem) => {
    // Format tick labels as dates using a library like moment.js
    return `${tickItem.getHours()}:${('0' + tickItem.getMinutes()).slice(-2)}`;
  };

  const formatYAxis = (tickItem) => {
    return parseFloat(tickItem).toFixed(3)
  };

  const formatTooltip = (value, name, props) => {
    // Format tooltip values using a library like numeral.js
    return [`${name}: ${parseFloat(value).toFixed(3)} `];
  };

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
  const history = cryptocurrency.aggregatedHistory.slice().map((value) => ({
    date: new Date(value.timestamp), ...value
  })).reverse();


  return (
    <div className="container" >
      <div className="details">
        <AssetInfo assetData={{ ...cryptocurrency, ...priceInfo }} />
        <ResponsiveContainer width="100%" height={500} >
          <LineChart data={history}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="date" tickFormatter={formatXAxis} tick={{ fontSize: '10px' }} />
            <YAxis type="number" domain={['dataMin', 'dataMax']} tickFormatter={formatYAxis} tick={{ fontSize: '10px' }} />
            <Tooltip formatter={formatTooltip} />
            <Legend />
            <Line
              type="monotone"
              name='First Quartile'
              dataKey="firstQuartilePrice"
              stroke={"#8884d8"}
              strokeWidth={2}
              dot={false}
            />
            <Line
              type="monotone"
              name='Third Quartile'
              dataKey="thirdQuartilePrice"
              stroke={"#ffc658"}
              strokeWidth={2}
              dot={false}
            />
            <Line
              type="monotone"
              name='Median'
              dataKey="medianPrice"
              stroke={"#82ca9d"}
              strokeWidth={2}
              dot={false}
            />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div >
  );
};

export default AssetPage;
