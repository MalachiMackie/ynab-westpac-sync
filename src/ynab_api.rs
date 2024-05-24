use anyhow::bail;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

use crate::YNAB_TOKEN;

pub async fn get_budgets() -> Result<Box<[Budget]>, anyhow::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.ynab.com/v1/budgets")
        .bearer_auth(YNAB_TOKEN)
        .send()
        .await?
        .json::<YnabResponse<BudgetsResponse>>()
        .await?;

    let Some(budgets) = response.data else {
        if let Some(error) = response.error {
            bail!("{}: {}", error.name, error.detail);
        } else {
            bail!("Failed to get budgets with an unknown error");
        }
    };

    Ok(budgets.budgets)
}

#[derive(Deserialize, Debug)]
pub struct YnabResponse<T> {
    data: Option<T>,
    error: Option<YnabError>,
}

#[derive(Deserialize, Debug)]
pub struct YnabError {
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
    id: String,
    name: String,
    last_modified_on: String,
    first_month: NaiveDate,
    last_month: NaiveDate,
    accounts: Option<Box<[Account]>>,
}

#[derive(Deserialize, Debug)]
pub struct Account {
    id: String,
    name: String,
    #[serde(rename = "type")]
    account_type: String,
    on_budget: bool,
    closed: bool,
    node: String,
    balance: f32,
    cleared_balance: f32,
    uncleared_balance: f32,
    transfer_payee_id: String,
    direct_import_linked: bool,
    direct_import_in_error: bool,
    last_reconciled_at: DateTime<Utc>,
    debt_original_balance: f32,
    deleted: bool,
}