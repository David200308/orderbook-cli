use serde::Deserialize;

use super::{OrderBook, OrderLevel};

// ─── Internal deserialization types ──────────────────────────

#[derive(Deserialize, Debug)]
struct HlLevel {
    px: String,
    sz: String,
    #[allow(dead_code)]
    n: u64,
}

#[derive(Deserialize, Debug)]
struct HlBook {
    coin: String,
    time: u64,
    levels: Vec<Vec<HlLevel>>, // [bids (desc), asks (asc)]
}

// ─── HTTP fetch ──────────────────────────────────────────────

const API_URL: &str = "https://api.hyperliquid.xyz/info";

pub async fn fetch_orderbook(coin: &str) -> Result<OrderBook, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "type": "l2Book",
        "coin": coin.to_uppercase()
    });
    let resp = client
        .post(API_URL)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;
    let raw = resp.json::<HlBook>().await?;

    let to_levels = |levels: &[HlLevel]| -> Vec<OrderLevel> {
        levels
            .iter()
            .map(|l| OrderLevel { price: l.px.clone(), size: l.sz.clone() })
            .collect()
    };

    let bids = raw.levels.first().map(|l| to_levels(l)).unwrap_or_default();
    let asks = raw.levels.get(1).map(|l| to_levels(l)).unwrap_or_default();

    Ok(OrderBook {
        market: "hyperliquid".to_string(),
        asset_id: raw.coin,
        timestamp: raw.time.to_string(),
        bids,
        asks,
        min_order_size: None,
        tick_size: None,
        neg_risk: None,
        last_trade_price: None,
    })
}
