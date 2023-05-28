import React, { useState } from "react";
import Highcharts from "highcharts";
import HighchartsReact from "highcharts-react-official";
import Loading from "../common/Loading";
import { useQuery, gql } from "@apollo/client";

const GRAPHQL_REQUEST = gql`
query getChartData($crypto_id: Int!) {
    source {
        fullHistory(cryptoId: $crypto_id) {
            aggregatedPrices {
                id
                timestamp
                medianPrice
                firstQuartilePrice,
                thirdQuartilePrice
            }
            sources {
                id
                name
                cryptoHistory(cryptoId: $crypto_id) {
                    id
                    price
                    timestamp
                }
            }
        }
    }
}
`;

const CryptoChart = ({ crypto_id }) => {
    const [sources, setSources] = useState(null);
    const [activeMap, setActive] = useState(new Map());

    const onCompleted = (data) => {

        const process_point = (price, timestamp) => {
            return { price: parseFloat(price), timestamp: new Date(timestamp).getTime() };
        };
        const processed_data = data.source.fullHistory.sources.map((source) => {

            const cryptoHistory = source.cryptoHistory.slice().map((point) => {
                return process_point(point.price, point.timestamp);
            });
            return { ...source, cryptoHistory, active: activeMap.get(source.name) === undefined ? false : activeMap.get(source.name), color: stringToColour(source.name) };
        });

        var median = [];
        var firstQuartile = [];
        var thirdQuartile = [];
        data.source.fullHistory.aggregatedPrices.slice().forEach((data) => {
            median.push(process_point(data.medianPrice, data.timestamp));
            firstQuartile.push(process_point(data.firstQuartilePrice, data.timestamp));
            thirdQuartile.push(process_point(data.thirdQuartilePrice, data.timestamp));
        });


        setSources(processed_data.concat(
            [{
                name: "Third Quartile",
                cryptoHistory: thirdQuartile,
                active: activeMap.get("Third Quartile") === undefined ? true : activeMap.get("Third Quartile"),
                color: '#ffc658'
            },
            {
                name: "Median",
                cryptoHistory: median,
                active: activeMap.get("Median") === undefined ? true : activeMap.get("Median"),
                color: '#82ca9d'
            },
            {
                name: "First Quartile",
                cryptoHistory: firstQuartile,
                active: activeMap.get("First Quartile") === undefined ? true : activeMap.get("First Quartile"),
                color: "#8884d8",
            },
            ]));
    };

    const { loading, error } = useQuery(GRAPHQL_REQUEST, { variables: { crypto_id }, onCompleted, pollInterval: 10000 });

    if (loading) { return <Loading />; }
    if (error) { return <p>Error: {error.message}</p>; }
    if (!sources) { return <Loading />; }

    const axisOptions = {
        tickColor: 'white',
        lineColor: 'white',
        labels: {
            style: {
                color: 'white',
            },
        }
    };

    const options = {
        chart: {
            backgroundColor: 'rgba(255, 255, 255, 0.0)',
            zoomType: 'xy',
            panning: {
                enabled: true,
                type: 'xy',
            },
            panKey: 'shift',
        },
        title: {
            text: "Price History",
            aligh: "left",
            style: {
                color: 'white',
            },
        },
        subtitle: {
            text:
                'Click and drag in the plot area to zoom in. Use shift + click to pan.',
            style: { color: 'white' },
            align: 'left'
        },
        xAxis: {
            type: "datetime",
            ...axisOptions,

        },
        yAxis: {
            panningEnabled: true,
            allowDecimals: false,
            title: {
                text: "Price data",
            },
            ...axisOptions,
            tickInterval: sources[0].cryptoHistory[0].price < 1. ? 0.005 : undefined,
            type: 'linear'

        },
        legend: {
            enabled: true,
            itemStyle: {
                color: 'white',
            }
        },
        tooltip: {
            valueDecimals: 3,
            shared: true
        },
        credits: {
            enabled: false
        },
        series: sources.map((source) => {
            return {
                name: source.name,
                data: source.cryptoHistory.slice().reverse().map((point) => {
                    return [point.timestamp, parseFloat(point.price)];
                }),
                color: source.color,
                type: 'areaspline',
                stickyTracking: false,
                visible: activeMap.get(source.name) === undefined ? source.active : activeMap.get(source.name),
                fillColor: {
                    linearGradient: {
                        x1: 0,
                        y1: 0,
                        x2: 0,
                        y2: 1
                    },
                    stops: [
                        [0, source.color],
                        [1, Highcharts.color(source.color).setOpacity(0).get('rgba')]
                    ]
                },
                marker: {
                    radius: 2
                },
                lineWidth: 2,
                states: {
                    hover: {
                        lineWidth: 3
                    }
                },
                threshold: null,
                events: {
                    legendItemClick: function () {
                        setActive(new Map(activeMap.set(this.name, !this.visible)));
                    }
                }
            };
        }),
    };

    return (
        <HighchartsReact highcharts={Highcharts} options={options} containerProps={{
            style: {
                width: '90%',
                height: '40vh'
            }
        }} />

    );
};

var stringToColour = function (str) {
    var hash = 0;
    for (var i = 0; i < str.length; i++) {
        hash = str.charCodeAt(i) + ((hash << 5) - hash);
    }
    var colour = '#';
    for (var i = 0; i < 3; i++) {
        var value = (hash >> (i * 8)) & 0xFF;
        colour += ('00' + value.toString(16)).substr(-2);
    }
    return colour;
}

export default CryptoChart;