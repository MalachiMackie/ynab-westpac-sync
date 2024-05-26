mod akahu_api;
mod ynab_api;

use std::io::stdin;

use akahu_api::akahu_get_accounts;
use tokio::task::spawn_blocking;
use ynab_api::ynab_get_accounts;

use crate::{
    akahu_api::{akahu_get_me, akahu_get_transactions},
    ynab_api::{ynab_get_budgets, ynab_get_payees},
};

pub const AKAHU_USER_TOKEN: &str = "";
pub const AKAHU_APP_TOKEN: &str = "";
pub const YNAB_TOKEN: &str = "";

#[tokio::main]
async fn main() {
    let akahu_accounts = akahu_get_accounts().await.unwrap();
    let ynab_budgets = ynab_get_budgets().await.unwrap();

    println!("Please select which budget you want to process:");
    for (i, budget) in ynab_budgets.into_iter().enumerate() {
        println!("{}: {}", i + 1, budget.name);
    }

    let selected_budget_index = spawn_blocking(read_u32).await.unwrap().unwrap();

    if selected_budget_index == 0 || selected_budget_index as usize > ynab_budgets.len() {
        return;
    }
    let Some(selected_budget) = ynab_budgets.get(selected_budget_index as usize - 1) else {
        println!("Please enter a number between 1 and {}", ynab_budgets.len());
        return;
    };

    let ynab_accounts = ynab_get_accounts(&selected_budget.id).await.unwrap();

    println!("Here are your accounts:");
    for account in ynab_accounts.into_iter() {
        println!("{}: ${:.2}", account.name, account.balance as f32 / 100.);
    }
}

fn read_u32() -> anyhow::Result<u32> {
    let mut buf = String::new();

    stdin().read_line(&mut buf)?;

    Ok(buf.trim().parse::<u32>()?)
}

// X get akahu accounts
// X get akahu transactions
// X get ynab accounts
// X get ynab transactions
// X get ynab payees
// - link akahu accounts to ynab accounts
// - find akahu transactions that aren't in ynab
//  - ?
// - post ynab transactions
// MVP finished
// - get ynab categories
// - create rules for auto-categorization
// - filter ynab transactions that dont have categories
// - find categories using rules
