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

// 1. get akahu accounts - done
// 2. get akahu transactions - done
// 3. get ynab accounts - done
// 4. get ynab transactions - done
// 5. link akahu accounts to ynab accounts
// 6. find akahu transactions that aren't in ynab
//  - ?
// 7. post ynab transactions
// MVP finished
// 8. get ynab categories
// 9. create rules for auto-categorization
// 10. filter ynab transactions that dont have categories
// 11. try and find categories using rules
