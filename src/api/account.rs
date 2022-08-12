pub mod withdrawals;
mod documents;

use std::fmt;

use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;

use crate::{client::Client, error::Error};

#[derive(Deserialize, Debug)]
pub struct AccountInformation<T> {
    pub time: String,
    pub mode: String,
    pub status: String,
    pub results: T,
}

/// Struct for account information.
// TODO: Make periods into chrono types
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountResults {
    pub created_at: String,
    pub account_id: String,
    pub firstname: String,
    pub lastname: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub billing_address: Option<String>,
    pub billing_email: Option<String>,
    pub billing_name: Option<String>,
    pub billing_vat: Option<String>,
    pub mode: String,
    pub deposit_id: Option<String>,
    pub client_id: Option<String>,
    pub account_number: Option<String>,
    pub iban_brokerage: Option<String>,
    pub iban_origin: Option<String>,
    pub bank_name_origin: Option<String>,
    pub balance: i64,
    pub cash_to_invest: i64,
    pub cash_to_withdraw: i64,
    pub amount_bought_intraday: i64,
    pub amount_sold_intraday: i64,
    pub amount_open_orders: i64,
    pub amount_open_withdrawals: i64,
    pub amount_estimate_taxes: i64,
    pub approved_at: Option<String>,
    pub trading_plan: String,
    pub data_plan: String,
    pub tax_allowance: Option<i64>,
    pub tax_allowance_start: Option<String>,
    pub tax_allowance_end: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PaginationResponse<T> {
    pub time: String,
    pub status: String,
    pub mode: String,
    pub results: Option<Vec<T>>,
    pub previous: Option<String>,
    pub next: Option<String>,
    pub total: i64,
    pub page: i32,
    pub pages: i32,
}





#[derive(Deserialize, Serialize, Debug)]
pub enum Sorting {
    #[serde(rename = "asc_")]
    Asc,
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
