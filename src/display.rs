use colored::*;

use crate::data::polymarket::OrderBook;

const WIDTH: usize = 58;

// ─── Internal helpers ────────────────────────────────────────

fn trunc(s: &str, max: usize) -> &str {
    if s.len() > max {
        &s[..max]
    } else {
        s
    }
}

fn parse(s: &str) -> f64 {
    s.parse().unwrap_or(0.0)
}

fn bar(ch: &str) -> ColoredString {
    ch.repeat(WIDTH).bright_cyan()
}

fn data_row(size: f64, price: f64, total: f64) -> String {
    // Pad to exactly WIDTH so the background fill always covers the full line
    format!("  {:>16.4}  {:>10.4}  {:>16.4}  ", size, price, total)
}

// Print a row with a depth-bar background that fills left→right by fill_ratio
fn depth_row(text: &str, fg: (u8, u8, u8), bg: (u8, u8, u8), fill_ratio: f64) {
    let n = text.len();
    let fill = ((fill_ratio * n as f64) as usize).clamp(1, n);
    print!(
        "{}{}",
        text[..fill].truecolor(fg.0, fg.1, fg.2).on_truecolor(bg.0, bg.1, bg.2),
        text[fill..].truecolor(fg.0, fg.1, fg.2),
    );
    println!();
}

// Interpolate between two u8 values by factor t in [0.0, 1.0]
fn lerp(a: u8, b: u8, t: f64) -> u8 {
    (a as f64 + (b as f64 - a as f64) * t.clamp(0.0, 1.0)) as u8
}

// Ask depth color: display_idx 0 = top (furthest, dim), total-1 = bottom (closest, vivid)
fn ask_color(display_idx: usize, total: usize) -> (u8, u8, u8) {
    let t = if total <= 1 { 1.0 } else { display_idx as f64 / (total - 1) as f64 };
    (lerp(100, 255, t), lerp(15, 60, t), lerp(15, 60, t))
}

// Bid depth color: display_idx 0 = top (closest, vivid), total-1 = bottom (furthest, dim)
fn bid_color(display_idx: usize, total: usize) -> (u8, u8, u8) {
    let t = if total <= 1 { 1.0 } else { 1.0 - display_idx as f64 / (total - 1) as f64 };
    (lerp(15, 60, t), lerp(100, 255, t), lerp(15, 60, t))
}

fn clear_screen() {
    print!("\x1B[H\x1B[2J");
}

// ─── Public render function ──────────────────────────────────

pub fn render(book: &OrderBook, market: &str, levels: usize, clear: bool) {
    if clear {
        clear_screen();
    }

    print_header(book, market);
    print_columns();
    print_asks(book, levels);
    print_spread(book);
    print_bids(book, levels);
    print_footer(book);
}

// ─── Sections ────────────────────────────────────────────────

fn print_header(book: &OrderBook, market: &str) {
    println!("{}", bar("═"));
    let title = format!("{}  ORDERBOOK  CLI", market.to_uppercase());
    println!(
        "{}",
        format!("{:^width$}", title, width = WIDTH)
            .bright_cyan()
            .bold()
    );
    println!("{}", bar("═"));

    println!("  Asset:  {}", trunc(&book.asset_id, WIDTH - 12).dimmed());
    println!("  Market: {}", trunc(&book.market, WIDTH - 12).dimmed());

    let mut meta: Vec<String> = Vec::new();
    if let Some(ref ltp) = book.last_trade_price {
        if !ltp.is_empty() && ltp != "0" && ltp != "0.0" {
            meta.push(format!("Last: {}", ltp.yellow().bold()));
        }
    }
    if let Some(ref tick) = book.tick_size {
        meta.push(format!("Tick: {}", tick.bright_white()));
    }
    if let Some(ref min_sz) = book.min_order_size {
        meta.push(format!("Min: {}", min_sz.bright_white()));
    }
    if book.neg_risk == Some(true) {
        meta.push("⚠ NEG-RISK".red().bold().to_string());
    }
    if !meta.is_empty() {
        println!("  {}", meta.join("  │  "));
    }

    println!("{}", bar("═"));
}

fn print_columns() {
    println!(
        "{}",
        format!(
            "  {:>16}  {:>10}  {:>16}",
            "SIZE", "PRICE", "TOTAL (USDC)"
        )
        .bright_white()
        .bold()
    );
    println!("{}", bar("─"));
}

fn print_asks(book: &OrderBook, levels: usize) {
    if book.asks.is_empty() {
        println!(
            "{}",
            format!("{:^width$}", "— no asks —", width = WIDTH).dimmed()
        );
        return;
    }
    let slice: Vec<_> = book.asks.iter().take(levels).collect();
    let total = slice.len();
    let max_size = slice.iter().map(|o| parse(&o.size)).fold(0.0_f64, f64::max);

    // API price-ascending → reverse display so highest ask is at top.
    // display_idx 0 = top (furthest, dim) → total-1 = bottom (closest, vivid)
    for (display_idx, ask) in slice.iter().rev().enumerate() {
        let size = parse(&ask.size);
        let price = parse(&ask.price);
        let fg = ask_color(display_idx, total);
        let fill_ratio = if max_size > 0.0 { size / max_size } else { 0.0 };
        depth_row(&data_row(size, price, size * price), fg, (55, 8, 8), fill_ratio);
    }
}

fn print_spread(book: &OrderBook) {
    let label = match (book.asks.first(), book.bids.first()) {
        (Some(ask), Some(bid)) => {
            let spread = parse(&ask.price) - parse(&bid.price);
            format!("──  SPREAD  {:.4}  ──", spread)
        }
        _ => "──  NO SPREAD  ──".to_string(),
    };
    println!(
        "{}",
        format!("{:^width$}", label, width = WIDTH).yellow()
    );
}

fn print_bids(book: &OrderBook, levels: usize) {
    if book.bids.is_empty() {
        println!(
            "{}",
            format!("{:^width$}", "— no bids —", width = WIDTH).dimmed()
        );
        return;
    }
    let slice: Vec<_> = book.bids.iter().take(levels).collect();
    let total = slice.len();
    let max_size = slice.iter().map(|o| parse(&o.size)).fold(0.0_f64, f64::max);

    // API price-descending; highest bid is first (nearest spread).
    // display_idx 0 = top (closest, vivid) → total-1 = bottom (furthest, dim)
    for (display_idx, bid) in slice.iter().enumerate() {
        let size = parse(&bid.size);
        let price = parse(&bid.price);
        let fg = bid_color(display_idx, total);
        let fill_ratio = if max_size > 0.0 { size / max_size } else { 0.0 };
        depth_row(&data_row(size, price, size * price), fg, (8, 55, 8), fill_ratio);
    }
}

fn print_footer(book: &OrderBook) {
    println!("{}", bar("═"));
    println!("  Updated: {}", book.timestamp.dimmed());
}
