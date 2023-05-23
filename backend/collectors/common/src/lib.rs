use anyhow::Context;
use csb_db_crypto::models::SourceCryptoMapping;

pub struct Collector {
    context: csb_db_crypto::Db,
    required_keys: Vec<SourceCryptoMapping>,
    source_id: i32,
    currency_id: i32,
    url: String,
    helper: Box<dyn CollectorHelper + Send + Sync>,
}

impl Collector {
    pub fn new(
        context: csb_db_crypto::Db,
        required_keys: Vec<SourceCryptoMapping>,
        currency_id: i32,
        url: String,
        helper: Box<dyn CollectorHelper + Send + Sync>,
    ) -> anyhow::Result<Self> {
        let source_id = required_keys
            .first()
            .context("Expect at least one ky")?
            .source_id;
        Ok(Self {
            context,
            required_keys,
            source_id,
            currency_id,
            url,
            helper,
        })
    }

    async fn update(&self) -> anyhow::Result<()> {
        let response = reqwest::get(&self.url).await?;
        let json = response.json::<serde_json::Value>().await?;
        let mut connection = self.context.db_connection.get().await?;
        let timestamp = chrono::Utc::now().naive_utc();
        let mut error = false;
        for crypto in &self.required_keys {
            let price = self
                .helper
                .retrieve_price_from_json(&json, &crypto.source_key);
            if let Err(e) = price {
                error = true;
                eprintln!("Failed to retrieve price: {e}");
                continue;
            }
            let price = csb_db_crypto::models::NewPrice::new(
                crypto.crypto_id,
                self.source_id,
                self.currency_id,
                price.unwrap(), // Safe to unwrap because we checked for error above
                timestamp,
            );
            if let Err(e) = price.insert(&mut connection).await {
                error = true;
                eprintln!("Failed to insert price: {e}: {price:?}");
            }
        }
        if error {
            Err(anyhow::anyhow!("Failed to insert price"))
        } else {
            Ok(())
        }
    }

    pub async fn run(&self) {
        loop {
            if let Err(e) = self.update().await {
                eprintln!(
                    "Failed to update cryptocurrencies: {e} with {}",
                    self.source_id
                );
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    }
}

pub trait CollectorHelper {
    fn create_url(&self, required_keys: &[SourceCryptoMapping]) -> String;
    fn retrieve_price_from_json(
        &self,
        json: &serde_json::Value,
        key: &str,
    ) -> anyhow::Result<bigdecimal::BigDecimal>;
}

pub async fn run(
    connection_string: String,
    source_name: &str,
    helper: Box<dyn CollectorHelper + Send + Sync>,
) -> anyhow::Result<()> {
    let context = csb_db_crypto::Db::new(connection_string)
        .await
        .expect("Failed to connect to the database");
    let required_keys = csb_db_crypto::models::SourceCryptoMapping::load_keys_by_source_name(
        &mut context
            .db_connection
            .get()
            .await
            .expect("Expected to get a connection from the pool"),
        source_name,
    )
    .await
    .expect("Failed to load keys");

    if required_keys.is_empty() {
        anyhow::bail!("No keys found for source {}", source_name);
    }
    let url = helper.create_url(&required_keys);
    Collector::new(context, required_keys, 1, url, helper)?
        .run()
        .await;
    Ok(())
}
