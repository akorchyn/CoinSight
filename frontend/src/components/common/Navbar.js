// src/components/Navbar.js

import React from 'react';
import { Link } from 'react-router-dom';
import { styled } from 'styletron-react';
import './Navbar.css';
import Search from './Search';
import SignOut from './SignOut';

const Centered = styled('div', {
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    height: '100%',
    width: '50%',
});

const Navbar = () => {
    const [token, setToken] = React.useState(null);

    React.useEffect(() => {
        const updateToken = () => {
            const token = localStorage.getItem('token');
            setToken(token);
        }

        window.addEventListener('storage', updateToken);
        updateToken();
    }, []);

    return (
        <nav className="navbar">
            <div className='logo-wrapper'>
                <img src="/icon.jpeg" alt="CoinSight" className="logo-icon" />
                <h1>
                    <Link to="/" className="logo">CoinSight</Link>
                </h1>
            </div>


            <Centered>
                <Search />
            </Centered>
            {token &&
                <ul>
                    <li>
                        <Link to="/alerts" className="navItem">Notifications</Link>
                    </li>
                    <li>
                        <SignOut className="navItem" />
                    </li>
                </ul>
            }
            {!token &&
                <ul>
                    <li>
                        <Link to="/login" className="navItem">Login</Link>
                    </li>
                    <li>
                        <Link to="/register" className="navItem">Register</Link>
                    </li>
                </ul>
            }
        </nav>
    );
};

export default Navbar;
