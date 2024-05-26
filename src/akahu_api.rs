use std::collections::HashMap;

use anyhow::{bail, ensure, Context};
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize};

use crate::{AKAHU_APP_TOKEN, AKAHU_USER_TOKEN};

const ME_ENDPOINT: &str = "v1/me";
const TRANSACTIONS_ENDPOINT: &str = "v1/transactions";
const ACCOUNTS_ENDPOINT: &str = "v1/accounts";

pub async fn akahu_get_transactions() -> Result<Box<[Transaction]>, anyhow::Error> {
    akahu_get_multiple::<Transaction>(TRANSACTIONS_ENDPOINT).await
}

pub async fn akahu_get_accounts() -> anyhow::Result<Box<[Account]>> {
    akahu_get_multiple::<Account>(ACCOUNTS_ENDPOINT).await
}

pub async fn akahu_get_me() -> Result<Me, anyhow::Error> {
    akahu_get_single::<Me>(ME_ENDPOINT).await
}

/// Me model. https://developers.akahu.nz/reference/get_me
#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct Me {
    #[serde(alias = "_id")]
    id: String,
    access_granted_at: DateTime<Utc>,
    email: String,
    mobile: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    preferred_name: Option<String>,
}

/// Account model. https://developers.akahu.nz/reference/get_accounts
#[derive(Deserialize, Debug)]
pub struct Account {
    #[serde(alias = "_id")]
    id: String,
    #[serde(alias = "_credentials")]
    credentials: String,
    connection: AccountConnection,
    name: String,
    status: AccountStatus,
    balance: Option<AccountBalance>,
    #[serde(alias = "type")]
    account_type: AccountType,
    attributes: Box<[AccountAttribute]>,
    formatted_account: Option<String>,
    meta: Option<HashMap<String, String>>,
    refreshed: AccountRefreshedObject,
}

#[derive(Deserialize, Debug)]
pub struct AccountRefreshedObject {
    balance: DateTime<Utc>,
    meta: DateTime<Utc>,
    transactions: DateTime<Utc>,
    party: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccountStatus {
    Active,
    Inactive,
}

#[derive(Deserialize, Debug)]
pub struct AccountBalance {
    currency: String,
    current: f32,
    available: f32,
    limit: Option<f32>,
    overdrawn: bool,
}

#[derive(Deserialize, Debug)]
pub struct AccountConnection {
    #[serde(alias = "_id")]
    id: String,
    name: String,
    logo: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccountType {
    Checking,
    Savings,
    CreditCard,
    Loan,
    Kiwisaver,
    Investment,
    TermDeposit,
    Foreign,
    Tax,
    Rewards,
    Wallet,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountAttribute {
    Transactions,
    TransferTo,
    TransferFrom,
    PaymentTo,
    PaymentFrom,
}

/// Transaction model. https://developers.akahu.nz/reference/get_transactions
#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Transaction {
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
pub enum TransactionType {
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

async fn akahu_get_single<T: DeserializeOwned>(endpoint: &str) -> Result<T, anyhow::Error> {
    let body = akahu_get_internal::<ResponseSingleItem<T>>(endpoint).await?;

    ensure!(
        body.success,
        body.message
            .unwrap_or(format!("Unknown error from {endpoint}"))
    );

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
        .bearer_auth(AKAHU_USER_TOKEN)
        .header("X-Akahu-ID", AKAHU_APP_TOKEN)
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
