use coindesk::Bitcoin;

#[tokio::main]
async fn main() {
    let data = Bitcoin::get().await.unwrap();

    println!("currency: {}, rate: {}, description: {}, symbol: {}", data.usd.code, data.usd.rate, data.usd.description, data.usd.symbol);
    println!("currency: {}, rate: {}, description: {}, symbol: {}", data.gbp.code, data.gbp.rate, data.gbp.description, data.gbp.symbol);
    println!("currency: {}, rate: {}, description: {}, symbol: {}", data.eur.code, data.eur.rate, data.eur.description, data.eur.symbol);
    println!("time: {}", data.time);
}