import React from "react";
import numeral from "numeral";

function AssetInfo({ assetData }) {
  const { name, description, medianPrice, firstQuartilePrice, thirdQuartilePrice } = assetData;
  console.log(assetData);
  return (
    <div>
      <h1>{name}</h1>
      <p>{description}</p>
      <p>Median Value: {numeral(medianPrice).format("$0,0.00")}</p>
      <p>
        Quartiles:{" "}
        <span>
          Q1: {numeral(firstQuartilePrice).format("$0,0.00")}
        </span>
        {" "}
        <span>
          Q3: {numeral(thirdQuartilePrice).format("$0,0.00")}
        </span>
      </p>
      {/* Add other relevant information here */}
    </div>
  );
}

export default AssetInfo;
