mod akahu_api;
mod ynab_api;

use akahu_api::akahu_get_accounts;
use ynab_api::get_accounts;

use crate::{
    akahu_api::{akahu_get_me, akahu_get_transactions},
    ynab_api::{get_budgets, get_payees},
};

pub const AKAHU_USER_TOKEN: &str = "";
pub const AKAHU_APP_TOKEN: &str = "";
pub const YNAB_TOKEN: &str = "";

#[tokio::main]
async fn main() {}

// 1. get akahu accounts
// 2. get akahu transactions
// 3. get ynab accounts
// 4. get ynab transactions
// 5. link akahu accounts to ynab accounts
