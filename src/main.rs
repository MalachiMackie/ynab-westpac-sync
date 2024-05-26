mod akahu_api;
mod ynab_api;

use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

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
    let (akahu_accounts, ynab_budgets) = tokio::join!(akahu_get_accounts(), ynab_get_budgets());
    let akahu_accounts = akahu_accounts.unwrap();
    let ynab_budgets = ynab_budgets.unwrap();

    println!("Your ynab budgets:");
    for (i, budget) in ynab_budgets.into_iter().enumerate() {
        println!("{}: {}", i + 1, budget.name);
    }
    print!("Please select which budget you want to process: ");

    flush_stdout().await.unwrap();

    let selected_budget_index = spawn_blocking(read_u32).await.unwrap().unwrap();
    println!();

    if selected_budget_index == 0 || selected_budget_index as usize > ynab_budgets.len() {
        return;
    }
    let Some(selected_budget) = ynab_budgets.get(selected_budget_index as usize - 1) else {
        println!("Please enter a number between 1 and {}", ynab_budgets.len());
        return;
    };

    let ynab_accounts = ynab_get_accounts(&selected_budget.id).await.unwrap();

    println!("Here are your ynab accounts:");
    for (i, account) in ynab_accounts.into_iter().enumerate() {
        println!(
            "{}: {} - ${:.2}",
            i + 1,
            account.name,
            account.balance as f32 / 100.
        );
    }

    let mut account_map = HashMap::<&str, &str>::new();
    println!("\nWe need to know which bank accounts map to which ynab accounts");
    println!("Using the ynab account number (eg. 1, 2 etc)...");
    for akahu_account in akahu_accounts.into_iter() {
        print!(
            "Which ynab account does this bank account map to? \"{}\" ",
            akahu_account.name
        );
        flush_stdout().await.unwrap();
        let index = spawn_blocking(read_u32).await.unwrap().unwrap();
        let Some(ynab_account) = ynab_accounts.get(index as usize - 1) else {
            println!(
                "Please enter a number between 1 and {}",
                ynab_accounts.len()
            );
            return;
        };
        println!();

        account_map.insert(&ynab_account.id, &akahu_account.id);
    }

    println!("{account_map:#?}")
}

async fn flush_stdout() -> anyhow::Result<()> {
    Ok(spawn_blocking(|| stdout().flush().unwrap()).await?)
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
// X link akahu accounts to ynab accounts
// - find akahu transactions that aren't in ynab
//  - ?
// - post ynab transactions
// MVP finished
// - get ynab categories
// - create rules for auto-categorization
// - filter ynab transactions that dont have categories
// - find categories using rules
