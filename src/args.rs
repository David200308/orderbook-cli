use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ob-cli")]
#[command(about = "Order book CLI viewer")]
#[command(long_about = "Fetches and displays the order book for a given market and token ID.")]
pub struct Args {
    /// Market to display: polymarket, hyperliquid (alias: hl), pendle (alias: pdl)
    #[arg(long, default_value = "polymarket")]
    pub market: String,

    /// Token ID / coin symbol (e.g. Polymarket token ID or "BTC" for Hyperliquid; prompted if omitted)
    #[arg(long = "tokenId")]
    pub token_id: Option<String>,

    /// Number of price levels to display per side
    #[arg(short = 'n', long, default_value = "10")]
    pub levels: usize,

    /// Auto-refresh interval in seconds (0 = single fetch)
    #[arg(short, long, default_value = "1")]
    pub refresh: u64,

    /// Use staging endpoint instead of production
    #[arg(long)]
    pub staging: bool,
}
