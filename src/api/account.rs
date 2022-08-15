use std::fmt;

use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;

use crate::{client::Client, error::Error};

mod documents;
/// Module for interacting with the withdrawal related endpoints
pub mod withdrawals;

///
#[derive(Deserialize, Debug)]
pub struct AccountInformation<T> {
    /// Timestamp of your API request
    pub time: String,
    /// Environment the request was placed in: "paper" or "money"
    // TODO: Make this an enum
    pub mode: String,
    /// API returns "ok" when account was successfully retrieved.
    pub status: String,
    /// The actual results of the query. Depends upon the given generics
    pub results: T,
}

/// Struct for account information.
// TODO: Make periods into chrono types
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountResults {
    /// Timestamp for when you created your account
    pub created_at: String,
    /// Unique Identification number for your account
    pub account_id: String,
    /// Your first name
    pub firstname: String,
    /// Your last name
    pub lastname: Option<String>,
    /// Your specified email address
    pub email: String,
    /// Your specified phone number
    pub phone: Option<String>,
    /// Your specified address
    pub address: Option<String>,
    /// The billing address you provided for your account
    pub billing_address: Option<String>,
    /// The billing email adress you provided for your account
    pub billing_email: Option<String>,
    /// The billing name you provided for your account
    pub billing_name: Option<String>,
    /// The billing VAT number you provided for your account
    pub billing_vat: Option<String>,
    /// The mode your account is currently on - "paper" or "money"
    // TODO: Make this an enum
    pub mode: String,
    /// Identification Number of your securities account
    pub deposit_id: Option<String>,
    /// The internal client identification number related to your account
    pub client_id: Option<String>,
    /// The account reference number
    pub account_number: Option<String>,
    /// The account reference number
    pub iban_brokerage: Option<String>,
    /// IBAN of the brokerage account at our partner bank.
    /// This is the IBAN you can transfer money from your referrence account to.
    pub iban_origin: Option<String>,
    /// Bank name your reference account is located a
    pub bank_name_origin: Option<String>,
    /// This is your account balance - the amount of money you have available in your brokerage account.
    /// The balance calculates as follows:
    /// (End-of-day balance from the day before) + (amount_sold_intraday)
    /// - (amount_bought_intraday) - (amount_open_withdrawals).
    pub balance: i64,
    /// This number shows you how much cash you have left to invest.
    ///
    /// The cash_to_invest calculates as follows:
    ///
    /// (balance) - (amount_open_orders)
    pub cash_to_invest: i64,
    /// This number shows you how much cash you have in your account to withdraw to your reference account.
    ///
    /// cash_to_withdraw is calculated as follows:
    ///
    /// (Your end-of-day balance from the day before) - (amount_bought_intraday)
    /// - (amount_open_withdrawals) - (amount_open_orders).
    pub cash_to_withdraw: i64,
    /// This is the intraday buy order amount.
    /// For example, if you bought 2 shares for 100€ each that day, the amount_bought_intraday
    /// would be 200€ (the API would return 2000000 in that case,
    /// see the [numbers page](https://docs.lemon.markets/numbers) for more information).
    pub amount_bought_intraday: i64,
    /// This is the intraday sell order amount.
    /// For example, if you sold 3 shares for 50€ each that day,
    /// the amount_sold_intraday would be 150€ (the API would return 1500000 in that case,
    /// see the [numbers page](https://docs.lemon.markets/numbers) for more information).
    pub amount_sold_intraday: i64,
    /// This is the intraday amount of open orders.
    // If you place an order that has amount X and its status is open,
    // the amount_open_orders would be the order amount
    // (+ the sum of all other open intraday orders).
    pub amount_open_orders: i64,
    /// For example, if you withdraw 500€ to your reference account,
    /// amount_open_withdrawals would return 5000000
    /// (see the [numbers page](https://docs.lemon.markets/numbers) for more information
    /// on the numbers format in the Trading API).
    pub amount_open_withdrawals: i64,
    /// This is the amount of estimated taxes (25%) for your intraday sell orders.
    /// For example, if you made a profit of 100€ from your intraday sell orders,
    /// the API would return 250000
    /// (see the [numbers page](https://docs.lemon.markets/numbers
    /// for more information in the Trading API
    pub amount_estimate_taxes: i64,
    /// Timestamp of live trading account approval
    pub approved_at: Option<String>,
    /// We offer different subscription plans for trading with lemon.markets.
    /// This endpoint tells you which plan you are currently on - go, investor, trader, or b2b.
    // TODO: Make this an enum
    pub trading_plan: String,
    /// We offer different subscription plans for trading with lemon.markets.
    /// This endpoint tells you which plan you are currently on - go, investor, trader, or b2b.
    // TODO: Make this an enum
    pub data_plan: String,
    /// Your tax tax allowance - between 0 and 801 €, as specified in your onboarding process
    pub tax_allowance: Option<i64>,
    /// Relevant start date for your tax allowance (usually 01/01/ of respective year)
    pub tax_allowance_start: Option<String>,
    /// Relevant end date for your tax allowance (usually 31/12/ of respective year)
    pub tax_allowance_end: Option<String>,
}

/// Enum for the different ways of sorting the results
#[derive(Deserialize, Serialize, Debug)]
pub enum Sorting {
    /// Use asc_ to sort your bank statements in ascending order (oldest ones first),
    #[serde(rename = "asc_")]
    Asc,
    /// Desc to sort your bank statements in descending order (newest ones first).
    #[serde(rename = "desc")]
    Desc,
}

impl fmt::Display for Sorting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match to_variant_name(self) {
            Ok(name) => write!(f, "{}", name),
            _ => write!(f, ""),
        }
    }
}

impl Client {
    /// Get account information
    pub fn get_account_information(&self) -> Result<AccountInformation<AccountResults>, Error> {
        const PATH: &str = "account";
        let resp = self.get::<AccountInformation<AccountResults>>(PATH);
        match resp {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use crate::*;

    #[test]
    fn test_get_account_information() {
        dotenv::dotenv().unwrap();
        let api_key = env::var("LEMON_MARKET_TRADING_API_KEY").unwrap();
        let client = client::Client::paper_client(&api_key);
        let resp = client.get_account_information().unwrap();
        assert_eq!(resp.status, "ok");
    }
}
