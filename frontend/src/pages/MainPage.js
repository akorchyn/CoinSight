import { useQuery } from '@apollo/client';
import gql from 'graphql-tag';
import React from 'react';
import CryptoList from '../components/main/CryptoList';
import './css/MainPage.css';
import Loading from '../components/common/Loading';

const GRAPHQL_REQUEST = gql`
    query getTopCryptos {
        crypto {
            top {
                id
                symbol
                name
                aggregatedHistory(limit: 10) {
                    medianPrice
                }
            }
        }
    }
`;

const MainPage = () => {
    const { loading, error, data } = useQuery(GRAPHQL_REQUEST, {
        pollInterval: 5000,
    });

    if (loading) return <Loading />;
    if (error) return <p>Error: {error.message}</p>;

    const cryptos = data?.crypto.top;

    return (
        <div>
            <p className='motto'>Navigate the world of crypto with confidence</p>
            <CryptoList cryptos={cryptos} />
        </div>
    );
};

export default MainPage;