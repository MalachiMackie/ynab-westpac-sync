use anyhow::bail;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{de::DeserializeOwned, Deserialize};

use crate::YNAB_TOKEN;

pub async fn get_budgets() -> anyhow::Result<Box<[Budget]>> {
    Ok(ynab_get::<BudgetsResponse>("budgets").await?.budgets)
}

pub async fn get_payees(budget_id: &str) -> anyhow::Result<Box<[Payee]>> {
    Ok(
        ynab_get::<PayeesResponse>(&format!("budgets/{budget_id}/payees"))
            .await?
            .payees,
    )
}

pub async fn get_accounts(budget_id: &str) -> anyhow::Result<Box<[Account]>> {
    Ok(
        ynab_get::<AccountsResponse>(&format!("budgets/{budget_id}/accounts"))
            .await?
            .accounts,
    )
}

async fn ynab_get<T: DeserializeOwned>(endpoint_relative_path: &str) -> anyhow::Result<T> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://api.ynab.com/v1/{endpoint_relative_path}"))
        .bearer_auth(YNAB_TOKEN)
        .send()
        .await?
        .json::<YnabResponse<T>>()
        .await?;

    let Some(data) = response.data else {
        if let Some(error) = response.error {
            bail!("{}: {}", error.name, error.detail);
        } else {
            bail!("Failed to get resource from {endpoint_relative_path}");
        }
    };

    Ok(data)
}

#[derive(Deserialize, Debug)]
struct YnabResponse<T> {
    data: Option<T>,
    error: Option<YnabError>,
}

#[derive(Deserialize, Debug)]
struct YnabError {
    id: String,
    name: String,
    detail: String,
}

#[derive(Deserialize, Debug)]
struct BudgetsResponse {
    budgets: Box<[Budget]>,
}

#[derive(Deserialize, Debug)]
pub struct Budget {
    pub id: String,
    pub name: String,
    pub last_modified_on: String,
    pub first_month: NaiveDate,
    pub last_month: NaiveDate,
    pub accounts: Option<Box<[Account]>>,
}

#[derive(Deserialize, Debug)]
pub struct AccountsResponse {
    accounts: Box<[Account]>,
}

#[derive(Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub name: String,
    #[serde(alias = "type")]
    pub account_type: String,
    pub on_budget: bool,
    pub closed: bool,
    pub note: Option<String>,
    pub balance: f32,
    pub cleared_balance: f32,
    pub uncleared_balance: f32,
    pub transfer_payee_id: String,
    pub direct_import_linked: bool,
    pub direct_import_in_error: bool,
    pub last_reconciled_at: Option<DateTime<Utc>>,
    pub debt_original_balance: Option<f32>,
    pub deleted: bool,
}

#[derive(Deserialize, Debug)]
struct PayeesResponse {
    payees: Box<[Payee]>,
}

#[derive(Deserialize, Debug)]
pub struct Payee {
    pub id: String,
    pub name: String,
    pub transfer_account_id: Option<String>,
    pub deleted: bool,
}
