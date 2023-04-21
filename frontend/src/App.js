// src/App.js

import './App.css';
import Navbar from './components/Navbar';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import CryptoList from './components/CryptoList';
import CryptoDetails from './components/CryptoDetails';
import Alerts from './components/Alerts';
import { mockedCryptoDetails } from './mockedData';

function App() {
  return (
    <Router>
      <div className="App">
        <Navbar />
        <Routes>
          <Route path="/" element={<CryptoList cryptos={mockedCryptoDetails} />} />
          <Route path="/details/:id" element={<CryptoDetails cryptos={mockedCryptoDetails} />} />
          <Route path="/alerts" element={<Alerts />} />
        </Routes>
      </div>
    </Router>
  );
}

export default App;
