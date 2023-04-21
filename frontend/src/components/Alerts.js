// src/components/Alerts.js

import React, { useState } from 'react';

const Alerts = () => {
    const [alertThreshold, setAlertThreshold] = useState('');

    const handleSubmit = (e) => {
        e.preventDefault();
        // Save the alert threshold and set up notifications
    };

    return (
        <div>
            <h2>Set Price Alert</h2>
            <form onSubmit={handleSubmit}>
                <label htmlFor="alertThreshold">Alert Threshold:</label>
                <input
                    type="number"
                    id="alertThreshold"
                    value={alertThreshold}
                    onChange={(e) => setAlertThreshold(e.target.value)}
                />
                <button type="submit">Set Alert</button>
            </form>
        </div>
    );
};

export default Alerts;
