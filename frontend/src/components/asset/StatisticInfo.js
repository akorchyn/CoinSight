import React from "react";
import "./StatisticInfo.css";
import Price from "../common/Price";

function AssetInfo({ assetData }) {
  const { name, symbol, description, medianPrice, firstQuartilePrice, thirdQuartilePrice } = assetData;
  return (
    <div className="wrapper">
      <h1 className="asset-name">{name}<span className="asset-symbol">{symbol}</span></h1>
      <p className="description">{description}</p>
      <div className="price-info-wrapper">
        <Price price={firstQuartilePrice} tooltip="Q1" minWidth={200} />
        <Price price={medianPrice} tooltip="Median Value" minWidth={200} />
        <Price price={thirdQuartilePrice} tooltip="Q3" minWidth={200} />
      </div>
      {/* Add other relevant information here */}
    </div>
  );
}

export default AssetInfo;
