use coindesk::Bitcoin;

#[tokio::main]
async fn main() {
    Bitcoin::get().await;
}