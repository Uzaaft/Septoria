use serde::{Deserialize, Serialize};

use crate::api::PaginationResponse;
use crate::client::Client;
use crate::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionPerformance {
    pub isin: String,
    pub isin_title: String,
    pub profit: i64,
    pub loss: i64,
    pub quantity_bought: i64,
    pub quantity_sold: i64,
    pub quantity_open: i64,
    pub opened_at: Option<String>,
    pub closed_at: Option<String>,
    pub fees: i64,
}
type PositionPerformancePagination = PaginationResponse<PositionPerformance>;

impl Client {
    /// Get an overview of your position performances
    ///  Using this endpoint, you can retrieve when positions were opened and closed,
    /// potential profits/losses, or related fees for position orders.
    pub fn get_positions_performance(&self) -> Result<PositionPerformancePagination, Error> {
        const PATH: &str = "positions/performance";
        let resp = self.get::<PositionPerformancePagination>(PATH);
        match resp {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }
}
#[cfg(test)]
mod tests {
    use std::env;

    use crate::client;

    #[test]
    fn test_get_positions_performance() {
        dotenv::dotenv().unwrap();
        let api_key = env::var("LEMON_MARKET_TRADING_API_KEY").unwrap();
        let client = client::Client::paper_client(&api_key);
        let positions = client.get_positions_performance().unwrap();
        dbg!(&positions);
        assert_eq!(positions.status, "ok");
    }
}
