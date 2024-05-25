mod akahu_api;
mod ynab_api;

use crate::{
    akahu_api::{akahu_get_me, akahu_get_transactions},
    ynab_api::{get_budgets, get_payees},
};

pub const AKAHU_USER_TOKEN: &str = "";
pub const AKAHU_APP_TOKEN: &str = "";
pub const YNAB_TOKEN: &str = "";

#[tokio::main]
async fn main() {}
