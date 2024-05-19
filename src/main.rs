#[tokio::main]
async fn main() {
    let transactions = get_transactions().await.expect("things to pass");
    println!("{transactions}");
}

async fn get_transactions() -> Result<String, anyhow::Error> {
    let client = reqwest::Client::new();
    let body = client
        // .get("https://api.akahu.io/v1/transactions")
        .get("https://api.akahu.io/v1/me")
        .bearer_auth("")
        .header("X-Akahu-ID", "")
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}
