// src/App.js

import './App.css';
import Navbar from './components/Navbar';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import CryptoDetails from './components/CryptoDetails';
import Alerts from './components/Alerts';
import { mockedCryptoDetails } from './mockedData';
import { ConfigProvider } from 'antd';
import MainPage from './pages/MainPage';
import Footer from './components/Footer';

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
          <div class="background-message">Coin Sight</div>
          <Routes>
            <Route path="/" element={<MainPage />} />
            <Route path="/details/:id" element={<CryptoDetails cryptos={mockedCryptoDetails} />} />
            <Route path="/alerts" element={<Alerts />} />
          </Routes>
          <Footer />
        </div>
      </Router>
    </ConfigProvider>
  );
}

export default App;
