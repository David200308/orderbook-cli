# Orderbook Display CLI Tool

A Orderbook 📖 CLI tool 🔨 build by Rust

<img src="img/demo.png" alt="demo" style="zoom:30%;" />

## Tech Stack

- Rust

## Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── img
│   └── demo.png
├── README.md
└── src
    ├── args.rs
    ├── data
    │   ├── mod.rs
    │   └── polymarket.rs
    ├── display.rs
    └── main.rs
```

## Support Markets

| Type              | Market     | Platform (Data Source) | Need API Key? |
| ----------------- | ---------- | ---------------------- | ------------- |
| Prediction Market | Polymarket | Polymarket             | ❌            |
