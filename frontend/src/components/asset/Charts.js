import React from "react";
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from "recharts";

function PriceHistoryChart({ data }) {
    const formatXAxis = (tickItem) => {
        // Format tick labels as dates using a library like moment.js
        return tickItem;
    };

    const formatTooltip = (value, name, props) => {
        // Format tooltip values using a library like numeral.js
        return [`${name}: ${value}`];
    };

    return (
        <ResponsiveContainer width="100%" height={500}>
            <LineChart data={data}>
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="date" tickFormatter={formatXAxis} />
                <YAxis />
                <Tooltip formatter={formatTooltip} />
                <Line
                    type="monotone"
                    dataKey="price"
                    stroke={"#8884d8"}
                    strokeWidth={2}
                    dot={true}
                />
            </LineChart>
        </ResponsiveContainer>
    );

}

export default PriceHistoryChart;