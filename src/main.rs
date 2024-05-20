mod akahu_api;
use crate::akahu_api::{akahu_get_me, akahu_get_transactions};

#[tokio::main]
async fn main() {
    let me = akahu_get_me().await.unwrap();
    println!("{me:?}");

    let transactions = akahu_get_transactions().await.unwrap();

    println!("{transactions:#?}");
}
