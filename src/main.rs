use anyhow::{bail, Context};
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize};

#[tokio::main]
async fn main() {
    let me = akahu_get::<Me>("v1/me").await.unwrap();
    println!("{me:?}");
}

async fn akahu_get<T: DeserializeOwned>(endpoint: &str) -> Result<T, anyhow::Error> {
    let client = reqwest::Client::new();
    let body = client
        .get(format!("https://api.akahu.io/{endpoint}"))
        .bearer_auth("")
        .header("X-Akahu-ID", "")
        .send()
        .await
        .with_context(|| format!("Fetching from {endpoint}"))?
        .json::<Response<T>>()
        .await
        .with_context(|| format!("Deserializing {} model", std::any::type_name::<T>()))?;

    if !body.success {
        bail!(body
            .message
            .unwrap_or(format!("Unknown error from {endpoint}")))
    }

    let Some(item) = body.item else {
        bail!("failed to deserialize {}", std::any::type_name::<T>())
    };

    Ok(item)
}

#[derive(Deserialize, Debug)]
struct Response<T> {
    success: bool,
    item: Option<T>,
    message: Option<String>,
}

#[allow(unused)]
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
