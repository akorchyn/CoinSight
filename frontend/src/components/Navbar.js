// src/components/Navbar.js

import React from 'react';
import { Link } from 'react-router-dom';
import './Navbar.css';
import Search from './Search';

const Navbar = () => {
    return (
        <nav className="navbar">
            <h1>
                <Link to="/" className="logo">CoinSight</Link>
            </h1>
            <Search></Search>

            <ul>
                <li>
                    <Link to="/" className="navItem">Home</Link>
                </li>
                <li>
                    <Link to="/alerts" className="navItem">Alerts</Link>
                </li>
            </ul>
        </nav>
    );
};

export default Navbar;
