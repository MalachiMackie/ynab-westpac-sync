mod api;
mod io;
mod process;

use process::process;

pub const AKAHU_USER_TOKEN: &str = "";
pub const AKAHU_APP_TOKEN: &str = "";
pub const YNAB_TOKEN: &str = "";

#[tokio::main]
async fn main() {
    process().await.unwrap();
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
