import { useQuery } from '@apollo/client';
import gql from 'graphql-tag';
import React from 'react';
import CryptoList from '../components/main/CryptoList';
import './css/MainPage.css';

const GRAPHQL_REQUEST = gql`
    query getTopCryptos {
        topCryptocurrencies {
            symbol,
            name,
            aggregatedHistory {
                medianPrice
            },
        }
    }
`;

const MainPage = () => {
    const { loading, error, data } = useQuery(GRAPHQL_REQUEST);
    if (loading) return null;
    if (error) return <p>Error: {error.message}</p>;

    const cryptos = data?.topCryptocurrencies;

    return (
        <div>
            <p className='motto'>Navigate the world of crypto with confidence</p>
            <CryptoList cryptos={cryptos} />
        </div>
    );
};

export default MainPage;