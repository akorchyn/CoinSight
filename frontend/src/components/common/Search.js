import { gql, useQuery } from "@apollo/client";
import React, { useState } from "react";
import { Input, SIZE } from "baseui/input";

import './Search.css';
import CryptoCurrencyCard from "./CryptoCurrencyCard";
import { Link } from "react-router-dom";

const SEARCH_QUERY = gql`
  query Search($query: String!) {
    search(query: $query) {
        name
        symbol
        aggregatedHistory(limit: 2) {
            medianPrice
        }
    }
  }
`;

const Search = () => {
    const [value, setValue] = useState("");
    const { loading, error, data } = useQuery(SEARCH_QUERY, {
        variables: { query: value.trim() },
    });

    const result = data?.search;

    const onInput = (e) => {
        setValue(e.target.value);
    };

    return (
        <div className="searchWrapper">
            <Input
                value={value}
                onChange={onInput}
                size={SIZE.large}
                placeholder="Search..."
                clearable
                clearOnEscape
            />
            <div className="searchResults">
                {loading && <p>Loading...</p>}
                {error && <p>Error: {error.message}</p>}
                {value.length > 0 && result && result.length == 0 && <p>No results</p>}
                {value.length > 0 && result && result.length > 0 && (
                    result.map((item) => {
                        const current_price = item.aggregatedHistory[0]?.medianPrice || 0;
                        const previousPrice = item.aggregatedHistory[1]?.medianPrice || 0;
                        return (
                            <Link key={item.symbol} to={`/asset/${item.symbol}`} style={{ textDecoration: 'none' }} onClick={() => setValue('')}>
                                <CryptoCurrencyCard key={item.symbol} assetName={item.name} symbol={item.symbol} currentPrice={current_price} previousPrice={previousPrice} />
                            </Link>
                        );
                    })
                )}
            </div>
        </div>
    );
};

export default Search;
