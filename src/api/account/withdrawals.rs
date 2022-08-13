use serde::{Deserialize, Serialize};

use crate::{client::Client, error::Error};

// TODO: This and the struct above could perhaps be merged into one...?
#[derive(Deserialize, Debug)]
pub struct WithdrawalResponse {
    pub time: String,
    pub mode: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WithdrawalRequest {
    amount: usize,
    pin: i64,
    idempotency: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Withdrawal {
    pub id: String,
    pub amount: i64,
    pub created_at: String,
    pub date: Option<String>,
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
    ) -> Result<WithdrawalResponse, Error> {
        const PATH: &str = "account/withdrawals/";
        let resp = self.get::<WithdrawalResponse>(PATH);
        match resp {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }

    /// Submit a new withdrawal
    /// TODO: Add docs for params
    pub fn post_withdrawal(
        &self,
        withdrawal: WithdrawalRequest,
    ) -> Result<WithdrawalResponse, Error> {
        const PATH: &str = "account/withdrawals/";
        let resp = self.post::<WithdrawalResponse, WithdrawalRequest>(PATH, withdrawal);
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
            amount: 100,
            pin: 1234,
            idempotency: None,
        };
        let resp = client.post_withdrawal(withdrawal).unwrap();
        dbg!(&resp);
        assert_eq!(resp.status, "ok");
    }
}
