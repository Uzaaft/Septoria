use serde::{Deserialize, Serialize};

use crate::api::Response;
use crate::{client::Client, error::Error};
use chrono::prelude::*;

/// Struct for the Withdrawal Request
#[derive(Serialize, Deserialize, Debug)]
pub struct WithdrawalRequest {
    /// Amount to withdraw
    amount: usize,
    /// PIN to use for withdrawal
    pin: i64,
    /// You can set your own unique idempotency key to prevent duplicate operations.
    /// Subsequent requests with the same idempotency key will then not go through and throw an error message.
    /// This means you cannot make the same withdrawal twice.
    idempotency: Option<String>,
}

///
#[derive(Deserialize, Serialize, Debug)]
pub struct Withdrawal {
    /// A unique Identification Number of your withdrawal
    pub id: String,
    /// The amount of the withdrawal
    pub amount: i64,
    /// Timestamp at which you created the withdrawal
    pub created_at: DateTime<Utc>,
    /// Timestamp at which the withdrawal was processed by our partner bank
    pub date: DateTime<Utc>,
    /// Your own unique idempotency key that you specified in your POST request to prevent
    /// duplicate withdrawals.
    pub idempotency: Option<String>,
}

impl Client {
    /// Get account withdrawls
    // TODO: Add support for pagination
    // TODO: Add docs for params
    pub fn get_account_withdrawls(
        &self,
        _limit: Option<i32>,
        _page: Option<i32>,
    ) -> Result<Response, Error> {
        const PATH: &str = "account/withdrawals/";
        let resp = self.get::<Response>(PATH);
        match resp {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }

    /// Submit a new withdrawal
    /// TODO: Add docs for params
    pub fn post_withdrawal(&self, withdrawal: WithdrawalRequest) -> Result<Response, Error> {
        const PATH: &str = "account/withdrawals/";
        let resp = self.post::<Response, WithdrawalRequest>(PATH, withdrawal);
        match resp {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::*;

    use super::*;

    #[test]
    fn test_get_account_withdrawls() {
        dotenv::dotenv().unwrap();
        let api_key = env::var("LEMON_MARKET_TRADING_API_KEY").unwrap();
        let client = client::Client::paper_client(&api_key);
        let resp = client.get_account_withdrawls(None, None).unwrap();
        dbg!(&resp);
        assert_eq!(resp.status, "ok");
    }

    #[test]
    // TODO: Figure out why this often return 400 error
    fn test_post_withdrawal() {
        dotenv::dotenv().unwrap();
        let api_key = env::var("LEMON_MARKET_TRADING_API_KEY").unwrap();
        let client = client::Client::paper_client(&api_key);
        let withdrawal = WithdrawalRequest {
            amount: 100, // 0.01 EUR
            pin: 1234,
            idempotency: None,
        };
        let resp = client.post_withdrawal(withdrawal).unwrap();
        assert_eq!(resp.status, "ok");
    }
}