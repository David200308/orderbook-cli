mod args;
mod data;
mod display;

use clap::Parser;
use colored::*;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let args = args::Args::parse();

    // Resolve token ID: CLI arg takes priority, otherwise prompt interactively
    let token_id = match args.token_id {
        Some(id) => id,
        None => {
            print!("{} ", "Input token id:".bright_cyan().bold());
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let trimmed = input.trim().to_string();
            if trimmed.is_empty() {
                eprintln!("{} token ID cannot be empty", "Error:".red().bold());
                std::process::exit(1);
            }
            trimmed
        }
    };

    loop {
        // Always clear and redraw from the top so the UI stays in place
        let clear = args.refresh > 0;

        match data::polymarket::fetch_orderbook(&token_id, args.staging).await {
            Ok(book) => display::render(&book, &args.market, args.levels, clear),
            Err(e) => {
                if clear {
                    print!("\x1B[H\x1B[2J");
                }
                eprintln!("{} {}", "Error:".red().bold(), e);
                if args.refresh == 0 {
                    std::process::exit(1);
                }
            }
        }

        if args.refresh == 0 {
            break;
        }

        // Countdown in-place on a single line
        for remaining in (1..=args.refresh).rev() {
            print!(
                "\r  {}  Refreshing in {}s  (Ctrl+C to quit)  ",
                "→".cyan(),
                remaining
            );
            std::io::stdout().flush().unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        // Clear the countdown line before redrawing
        print!("\r{}\r", " ".repeat(58));
        std::io::stdout().flush().unwrap();
    }
}
