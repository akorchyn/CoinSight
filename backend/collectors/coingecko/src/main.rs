use anyhow::Context;
use bigdecimal::{BigDecimal, FromPrimitive};
use csb_collector_common::CollectorHelper;
use csb_db::models::SourceCryptoMapping;

struct Urly {}
impl CollectorHelper for Urly {
    fn create_url(&self, cryptocurrencies: &[SourceCryptoMapping]) -> String {
        let param_string = cryptocurrencies
            .iter()
            .map(|c| c.source_key.clone())
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "https://api.coingecko.com/api/v3/simple/price?vs_currencies=usd&ids={param_string}"
        )
    }

    fn retrieve_price_from_json(
        &self,
        json: &serde_json::Value,
        symbol: &str,
    ) -> anyhow::Result<BigDecimal> {
        let symbol = json.get(symbol).context("Failed to get symbol")?;
        let price = symbol
            .get("usd")
            .context("Failed to get price")?
            .as_f64()
            .context("Failed to convert price to f64")?;
        let price =
            bigdecimal::BigDecimal::from_f64(price).context("Failed to convert to BigDecimal")?;
        Ok(price)
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    csb_collector_common::run(connection_string, "CoinGecko", Box::new(Urly {}))
        .await
        .unwrap()
}
