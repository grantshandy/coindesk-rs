# coindesk-rs
A Bitcoin price index API powered by [coindesk.com](https://coindesk.com).

```
coindesk = "0.1.0"
```

Because this is an async library you will need an async runtime like async_std or tokio.

Example:
```rust
use coindesk::Bitcoin;

#[tokio::main]
async fn main() {
    let data = Bitcoin::get().await.unwrap();

    println!("currency: {}, rate: {}, description: {}, symbol: {}", data.usd.code, data.usd.rate, data.usd.description, data.usd.symbol);
    println!("currency: {}, rate: {}, description: {}, symbol: {}", data.gbp.code, data.gbp.rate, data.gbp.description, data.gbp.symbol);
    println!("currency: {}, rate: {}, description: {}, symbol: {}", data.eur.code, data.eur.rate, data.eur.description, data.eur.symbol);
    println!("time: {}", data.time);
}
```

disclaimer: This data was produced from the CoinDesk Bitcoin Price Index (USD). Non-USD currency data converted using hourly conversion rate from [openexchangerates.org](https://openexchangerates.org)