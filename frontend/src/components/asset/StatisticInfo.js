import React from "react";
import numeral from "numeral";

function AssetInfo({ assetData }) {
  const { name, description, current_price, market_cap, median_value, quartile25, quartile75 } = assetData;

  return (
    <div>
      <h1>{name}</h1>
      <p>{description}</p>
      <p>Price: {numeral(current_price).format("$0,0.00")}</p>
      <p>Market Cap: {numeral(market_cap).format("$0,0")}</p>
      <p>Median Value: {numeral(median_value).format("$0,0.00")}</p>
      <p>
        Quartiles:{" "}
        <span>
          Q1: {numeral(quartile25).format("$0,0.00")}
        </span>
        {" "}
        <span>
          Q3: {numeral(quartile75).format("$0,0.00")}
        </span>
      </p>
      {/* Add other relevant information here */}
    </div>
  );
}

export default AssetInfo;
