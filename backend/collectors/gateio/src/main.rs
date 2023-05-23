use std::str::FromStr;

use anyhow::Context;
use bigdecimal::BigDecimal;
use csb_collector_common::CollectorHelper;
use csb_db_crypto::models::SourceCryptoMapping;

struct Urly {}
impl CollectorHelper for Urly {
    fn create_url(&self, _: &[SourceCryptoMapping]) -> String {
        "https://api.gateio.ws/api/v4/spot/tickers".to_string()
    }

    fn retrieve_price_from_json(
        &self,
        json: &serde_json::Value,
        symbol: &str,
    ) -> anyhow::Result<BigDecimal> {
        let json = json.as_array().context("Failed to get array")?;
        let symbol = format!("{}_USD", symbol);
        let json = json
            .iter()
            .find(|x| {
                x.get("currency_pair")
                    .map_or(false, |x| x.as_str().map_or(false, |x| x == symbol))
            })
            .context("Failed to find symbol")?;
        let price = json
            .as_object()
            .context("Failed to get object")?
            .get("last")
            .context("Failed to get price")?
            .as_str()
            .context("Failed to get price as string")?;
        let price =
            bigdecimal::BigDecimal::from_str(price).context("Failed to convert to BigDecimal")?;
        Ok(price)
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    csb_collector_common::run(connection_string, "GateIO", Box::new(Urly {}))
        .await
        .unwrap()
}
