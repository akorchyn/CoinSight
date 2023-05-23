// src/App.js

import './App.css';
import Navbar from './components/common/Navbar';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Alerts from './components/Alerts';
import { ConfigProvider } from 'antd';
import MainPage from './pages/MainPage';
import AssetPage from './pages/AssetPage';
import Footer from './components/common/Footer';
import { ApolloClient, ApolloProvider, InMemoryCache } from '@apollo/client';
import { Client as Styletron } from 'styletron-engine-atomic';
import { Provider as StyletronProvider } from 'styletron-react';
import { DarkTheme, BaseProvider, styled } from 'baseui';
import Login from './pages/Login';
import Registration from './pages/Registration';

const client = new ApolloClient({
  uri: process.env.REACT_APP_API_ENDPOINT,
  fetchOptions: {
    mode: 'no-cors',
  },
  cache: new InMemoryCache()
});

const engine = new Styletron();

function App() {
  return (
    <ApolloProvider client={client}>
      <StyletronProvider value={engine}>
        <BaseProvider theme={DarkTheme}>
          <ConfigProvider theme={{
            token: {
              colorBgBase: '#333333'
            },
          }}>
            <Router>
              <div className="App">
                <Navbar />
                <div className="background-message">Coin Sight</div>
                <Routes>
                  <Route path="/" element={<MainPage />} />
                  <Route path="/asset/:symbol" element={<AssetPage />} />
                  <Route path="/alerts" element={<Alerts />} />
                  <Route path="/login" element={<Login />} />
                  <Route path="/register" element={<Registration />} />
                </Routes>
                <Footer />
              </div>
            </Router>
          </ConfigProvider>
        </BaseProvider>
      </StyletronProvider>
    </ApolloProvider>

  );
}

export default App;
