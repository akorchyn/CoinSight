// src/App.js

import './App.css';
import Navbar from './components/common/Navbar';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Alerts from './components/Alerts';
import { mockedCryptoDetails } from './mockedData';
import { ConfigProvider } from 'antd';
import MainPage from './pages/MainPage';
import AssetPage from './pages/AssetPage';
import Footer from './components/common/Footer';
import { ApolloClient, ApolloProvider, InMemoryCache } from '@apollo/client';


const client = new ApolloClient({
  uri: process.env.REACT_APP_API_ENDPOINT,
  fetchOptions: {
    mode: 'no-cors',
  },
  cache: new InMemoryCache()
});

function App() {
  return (
    <ApolloProvider client={client}>
      <ConfigProvider theme={{
        token: {
          colorBgBase: '#333'
        },
      }}>
        <Router>
          <div className="App">
            <Navbar />
            <div className="background-message">Coin Sight</div>
            <Routes>
              <Route path="/" element={<MainPage />} />
              <Route path="/asset/:symbol" element={<AssetPage cryptos={mockedCryptoDetails} />} />
              <Route path="/alerts" element={<Alerts />} />
            </Routes>
            <Footer />
          </div>
        </Router>
      </ConfigProvider>
    </ApolloProvider>

  );
}

export default App;
