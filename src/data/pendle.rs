use serde::Deserialize;

use super::{OrderBook, OrderLevel};

const API_URL: &str = "https://api-v2.pendle.finance/core";
const DEFAULT_CHAIN_ID: u64 = 42161; // Arbitrum

// ─── Internal deserialization types ──────────────────────────

#[derive(Deserialize, Debug)]
struct PendleOrderEntry {
    #[serde(rename = "impliedApy")]
    implied_apy: f64,
    #[serde(rename = "limitOrderSize")]
    limit_order_size: String,
}

#[derive(Deserialize, Debug)]
struct PendleBook {
    #[serde(rename = "longYieldEntries")]
    long_yield_entries: Vec<PendleOrderEntry>,
    #[serde(rename = "shortYieldEntries")]
    short_yield_entries: Vec<PendleOrderEntry>,
}

// ─── HTTP fetch ──────────────────────────────────────────────

// token_id format: "{chainId}:{marketAddress}" or just "{marketAddress}" (defaults to Arbitrum)
// The "price" column shows APY in percent (e.g. 8.30 = 8.30% APY).
// Sizes are in token units (limitOrderSize ÷ 10^18).
pub async fn fetch_orderbook(token_id: &str) -> Result<OrderBook, Box<dyn std::error::Error>> {
    let (chain_id, market) = match token_id.find(':') {
        Some(pos) => {
            let (chain, rest) = token_id.split_at(pos);
            (chain.parse::<u64>().unwrap_or(DEFAULT_CHAIN_ID), &rest[1..])
        }
        None => (DEFAULT_CHAIN_ID, token_id),
    };

    let url = format!(
        "{}/v2/limit-orders/book/{}?market={}&precisionDecimal=2&limit=50",
        API_URL, chain_id, market
    );

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("Accept", "application/json")
        .send()
        .await?
        .error_for_status()?;
    let raw = resp.json::<PendleBook>().await?;

    let to_level = |entry: &PendleOrderEntry| -> OrderLevel {
        let size = entry.limit_order_size.parse::<f64>().unwrap_or(0.0) / 1e18;
        OrderLevel {
            // Store APY as a percentage so display.rs renders e.g. "8.3000"
            price: format!("{:.4}", entry.implied_apy * 100.0),
            size: format!("{:.4}", size),
        }
    };

    // longYieldEntries  = sellers of PT (asks).  API returns descending APY.
    //   asks[0] = highest APY (best ask, cheapest PT) → display.rs reversal puts it near spread. ✓
    // shortYieldEntries = buyers of PT (bids).  API returns ascending APY.
    //   bids[0] = lowest APY (best bid, priciest PT) → display.rs shows it at top near spread. ✓
    let asks = raw.long_yield_entries.iter().map(to_level).collect();
    let bids = raw.short_yield_entries.iter().map(to_level).collect();

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_default();

    Ok(OrderBook {
        market: "pendle".to_string(),
        asset_id: format!("{}:{}", chain_id, market),
        timestamp: ts,
        bids,
        asks,
        min_order_size: None,
        tick_size: None,
        neg_risk: None,
        last_trade_price: None,
    })
}
