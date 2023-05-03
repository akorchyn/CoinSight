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

function App() {
  return (
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
            <Route path="/asset/:id" element={<AssetPage cryptos={mockedCryptoDetails} />} />
            <Route path="/alerts" element={<Alerts />} />
          </Routes>
          <Footer />
        </div>
      </Router>
    </ConfigProvider>
  );
}

export default App;
