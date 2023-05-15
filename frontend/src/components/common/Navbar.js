// src/components/Navbar.js

import React from 'react';
import { Link } from 'react-router-dom';
import { styled } from 'styletron-react';
import './Navbar.css';
import Search from './Search';

const Centered = styled('div', {
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    height: '100%',
    width: '50%',
});

const Navbar = () => {
    return (
        <nav className="navbar">
            <h1>
                <Link to="/" className="logo">CoinSight</Link>
            </h1>


            <Centered>
                <Search />
            </Centered>
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
