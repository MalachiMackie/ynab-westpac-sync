use anyhow::{bail, Context};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let transactions = get_me().await.unwrap();
    println!("{transactions:?}");
}

async fn get_me() -> Result<Me, anyhow::Error> {
    let client = reqwest::Client::new();
    let body = client
        .get("https://api.akahu.io/v1/me")
        .bearer_auth("")
        .header("X-Akahu-ID", "")
        .send()
        .await
        .context("Fetching me")?
        .json::<Response<Me>>()
        .await
        .context("Deserializing me model")?;

    if !body.success {
        bail!(body.message.unwrap_or("Failed to get me".to_owned()))
    }

    let Some(item) = body.item else {
        bail!("failed to deserialize me")
    };

    Ok(item)
}

#[derive(Deserialize, Debug)]
struct Response<T> {
    success: bool,
    item: Option<T>,
    message: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Me {
    #[serde(alias = "_id")]
    id: String,
    access_granted_at: DateTime<Utc>,
    email: String,
    mobile: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    preferred_name: Option<String>,
}
