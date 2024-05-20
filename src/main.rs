use anyhow::{bail, ensure, Context};
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize};

#[tokio::main]
async fn main() {
    let me = akahu_get_single::<Me>("v1/me").await.unwrap();
    println!("{me:?}");

    let transactions = akahu_get_multiple::<Transaction>("v1/transactions")
        .await
        .unwrap();

    println!("{transactions:#?}");
}

async fn akahu_get_single<T: DeserializeOwned>(endpoint: &str) -> Result<T, anyhow::Error> {
    let body = akahu_get_internal::<ResponseSingleItem<T>>(endpoint).await?;

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

async fn akahu_get_multiple<T: DeserializeOwned>(
    endpoint: &str,
) -> Result<Box<[T]>, anyhow::Error> {
    let body = akahu_get_internal::<ResponseMultipleItems<T>>(endpoint).await?;

    ensure!(
        body.success,
        body.message
            .unwrap_or(format!("Unknown error from {endpoint}"))
    );

    Ok(body.items)
}

async fn akahu_get_internal<T: DeserializeOwned>(endpoint: &str) -> Result<T, anyhow::Error> {
    let client = reqwest::Client::new();
    client
        .get(format!("https://api.akahu.io/{endpoint}"))
        .bearer_auth("")
        .header("X-Akahu-ID", "")
        .send()
        .await
        .with_context(|| format!("Fetching from {endpoint}"))?
        .json::<T>()
        .await
        .with_context(|| format!("Deserializing {} model", std::any::type_name::<T>()))
}

#[derive(Deserialize, Debug)]
struct ResponseSingleItem<T> {
    success: bool,
    item: Option<T>,
    message: Option<String>,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct ResponseMultipleItems<T> {
    success: bool,
    items: Box<[T]>,
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

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Transaction {
    #[serde(alias = "_id")]
    id: String,
    #[serde(alias = "_account")]
    account: String,
    #[serde(alias = "_connection")]
    connection: String,
    #[serde(alias = "_user")]
    user: String,
    /// timestamp the transaction was retrieved and created by akahu
    #[serde(alias = "created_at")]
    akahu_created_at: DateTime<Utc>,
    /// timestamp the transaction was last updated by akahu
    #[serde(alias = "updated_at")]
    akahu_updated_at: DateTime<Utc>,
    #[serde(alias = "date")]
    transaction_date: DateTime<Utc>,
    description: String,
    amount: f32,
    /// the balance after the transaction
    balance: f32,
    #[serde(alias = "type")]
    transaction_type: TransactionType,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
enum TransactionType {
    Credit,
    Debit,
    Payment,
    Transfer,
    #[serde(alias = "STANDING ORDER")]
    StandingOrder,
    Eftpos,
    Interest,
    Fee,
    Tax,
    #[serde(alias = "CREDIT CARD")]
    CreditCard,
    #[serde(alias = "DIRECT DEBIT")]
    DirectDebit,
    Atm,
    Loan,
}
