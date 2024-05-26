use anyhow::bail;
use std::collections::HashMap;

use crate::{
    api::akahu_api::{self, akahu_get_accounts},
    api::ynab_api::{self, ynab_get_accounts, ynab_get_budgets, Budget},
    io::{flush_stdout, read_u32},
};

pub async fn process() -> anyhow::Result<()> {
    let (akahu_accounts, ynab_budgets) = tokio::join!(akahu_get_accounts(), ynab_get_budgets());
    let akahu_accounts = akahu_accounts?;
    let ynab_budgets = ynab_budgets?;

    let selected_budget = select_budget_to_process(&ynab_budgets).await?;

    let ynab_accounts = ynab_get_accounts(&selected_budget.id).await?;

    let account_map = get_account_map(&ynab_accounts, &akahu_accounts).await?;

    println!("{account_map:#?}");
    Ok(())
}

async fn select_budget_to_process(budgets: &[Budget]) -> anyhow::Result<&Budget> {
    println!("Your ynab budgets:");
    for (i, budget) in budgets.into_iter().enumerate() {
        println!("{}: {}", i + 1, budget.name);
    }
    print!("Please select which budget you want to process: ");

    flush_stdout().await?;

    let selected_budget_index = read_u32().await?;
    println!();

    let Some(selected_budget) = budgets.get(selected_budget_index as usize - 1) else {
        bail!("Please enter a number between 1 and {}", budgets.len());
    };

    Ok(selected_budget)
}

async fn get_account_map<'a>(
    ynab_accounts: &'a [ynab_api::Account],
    akahu_accounts: &'a [akahu_api::Account],
) -> anyhow::Result<HashMap<&'a str, &'a str>> {
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
        flush_stdout().await?;
        let index = read_u32().await?;
        let Some(ynab_account) = ynab_accounts.get(index as usize - 1) else {
            bail!(
                "Please enter a number between 1 and {}",
                ynab_accounts.len()
            );
        };
        println!();

        account_map.insert(&ynab_account.id, &akahu_account.id);
    }

    Ok(account_map)
}
