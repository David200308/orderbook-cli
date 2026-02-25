use serde::Deserialize;
use serde_json::Value;

// ─── Response types ──────────────────────────────────────────

#[derive(Deserialize, Debug)]
pub struct OrderLevel {
    pub price: String,
    pub size: String,
}

fn any_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s,
        Value::Number(n) => n.to_string(),
        Value::Null => String::new(),
        v => v.to_string(),
    })
}

#[derive(Deserialize, Debug)]
pub struct OrderBook {
    pub market: String,
    pub asset_id: String,
    #[serde(deserialize_with = "any_to_string")]
    pub timestamp: String,
    pub bids: Vec<OrderLevel>,
    pub asks: Vec<OrderLevel>,
    #[serde(default)]
    pub min_order_size: Option<String>,
    #[serde(default)]
    pub tick_size: Option<String>,
    #[serde(default)]
    pub neg_risk: Option<bool>,
    #[serde(default)]
    pub last_trade_price: Option<String>,
}

// ─── HTTP fetch ──────────────────────────────────────────────

const PROD_URL: &str = "https://clob.polymarket.com";
const STAGING_URL: &str = "https://clob-staging.polymarket.com";

pub async fn fetch_orderbook(
    token_id: &str,
    staging: bool,
) -> Result<OrderBook, Box<dyn std::error::Error>> {
    let base = if staging { STAGING_URL } else { PROD_URL };
    let url = format!("{}/book?token_id={}", base, token_id);
    let resp = reqwest::get(&url).await?.error_for_status()?;
    Ok(resp.json::<OrderBook>().await?)
}
