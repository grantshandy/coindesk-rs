use coindesk::Bitcoin;

#[tokio::main]
async fn main() {
    let data = Bitcoin::get().await.unwrap();

    println!("currency: {}, rate: {}", data.usd.code, data.usd.rate);
    println!("currency: {}, rate: {}", data.gbp.code, data.gbp.rate);
    println!("currency: {}, rate: {}", data.eur.code, data.eur.rate);
    println!("time: {}", data.time);
}