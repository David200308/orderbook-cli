use serde::Deserialize;
use serde_json::Value;

use super::{OrderBook, OrderLevel};

// ─── Internal deserialization types ──────────────────────────

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
struct RawOrderLevel {
    pub price: String,
    pub size: String,
}

#[derive(Deserialize, Debug)]
struct RawOrderBook {
    pub market: String,
    pub asset_id: String,
    #[serde(deserialize_with = "any_to_string")]
    pub timestamp: String,
    pub bids: Vec<RawOrderLevel>,
    pub asks: Vec<RawOrderLevel>,
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
    let raw = resp.json::<RawOrderBook>().await?;
    Ok(OrderBook {
        market: raw.market,
        asset_id: raw.asset_id,
        timestamp: raw.timestamp,
        bids: raw.bids.into_iter().map(|o| OrderLevel { price: o.price, size: o.size }).collect(),
        asks: raw.asks.into_iter().map(|o| OrderLevel { price: o.price, size: o.size }).collect(),
        min_order_size: raw.min_order_size,
        tick_size: raw.tick_size,
        neg_risk: raw.neg_risk,
        last_trade_price: raw.last_trade_price,
    })
}
