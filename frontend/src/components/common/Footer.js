import React from "react";
import './Footer.css'

const Footer = () => {
    const currentYear = new Date().getFullYear();

    return (
        <footer className="footer">
            <p>© {currentYear} CoinSight. All rights reserved.</p>
            <p>
                <a href="#">Terms of Service</a> | <a href="#">Privacy Policy</a>
            </p>
            <p className="footer-motto">Join the revolution. Invest in the future.</p>
        </footer>
    );
}

export default Footer;