use serde::{Deserialize, Serialize};

use crate::api::PaginationResponse;
use crate::{client::Client, error::Error};

mod performance;
mod statements;

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub isin: String,
    pub isin_title: String,
    pub quantity: i64,
    pub buy_price_avg: i64,
    pub estimated_price_total: i64,
    pub estimated_price: i64,
}

impl Client {
    /// Get all positions
    pub fn get_positions(&self) -> Result<PaginationResponse<Position>, Error> {
        const PATH: &str = "positions/";
        let resp = self.get::<PaginationResponse<Position>>(PATH);
        match resp {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod position_tests {
    use std::env;

    use crate::client::Client;
    use crate::*;

    use super::*;

    #[test]
    fn test_get_positions() {
        dotenv::dotenv().unwrap();
        let api_key = env::var("LEMON_MARKET_TRADING_API_KEY").unwrap();
        let client = client::Client::paper_client(&api_key);
        let client = Client::paper_client(&api_key);
        let positions = client.get_positions().unwrap();
        dbg!(&positions);
        assert_eq!(positions.status, "ok");
    }
}
