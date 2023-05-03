// src/components/Navbar.js

import React from 'react';
import { Link } from 'react-router-dom';
import styles from './Navbar.module.css';
import Search from './Search';

const Navbar = () => {
    return (
        <nav className={styles.navbar}>
            <h1 className={styles.logo}>
                <Link to="/" className={styles.logo}>CoinSight</Link>
            </h1>
            <Search></Search>

            <ul>
                <li>
                    <Link to="/" className={styles.navItem}>Home</Link>
                </li>
                <li>
                    <Link to="/alerts" className={styles.navItem}>Alerts</Link>
                </li>
            </ul>
        </nav>
    );
};

export default Navbar;
