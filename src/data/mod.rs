pub mod hyperliquid;
pub mod pendle;
pub mod polymarket;

// ─── Shared types ────────────────────────────────────────────

pub struct OrderLevel {
    pub price: String,
    pub size: String,
}

pub struct OrderBook {
    pub market: String,
    pub asset_id: String,
    pub timestamp: String,
    pub bids: Vec<OrderLevel>,
    pub asks: Vec<OrderLevel>,
    pub min_order_size: Option<String>,
    pub tick_size: Option<String>,
    pub neg_risk: Option<bool>,
    pub last_trade_price: Option<String>,
}
