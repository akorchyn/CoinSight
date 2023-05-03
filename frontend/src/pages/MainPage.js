import React from 'react';
import CryptoList from '../components/CryptoList';
import { mockedCryptoDetails } from '../mockedData';
import './css/MainPage.css';

const MainPage = () => {
    return (
        <div>
            <p className='motto'>Navigate the world of crypto with confidence</p>
            <CryptoList cryptos={mockedCryptoDetails} />
        </div>
    );
};

export default MainPage;