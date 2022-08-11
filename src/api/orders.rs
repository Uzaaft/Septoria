use serde::{Deserialize, Serialize};

use crate::{client::Client, error::Error};

#[derive(Deserialize, Debug)]
// The struct for placing an order - body of the post request
pub struct OrderPlacing {
    pub isin: String,
    pub expires_at: Option<String>,
    pub side: String, // "buy" or "sell"
    pub quantity: i64,
    pub venue: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
// The struct for placing an order - the response of the request
pub struct OrderPlacingResponse {
    pub time: String,
    pub status: String,
    pub mode: String,
    // pub results: Option<T>,

    // pub created_at: Option<String>,
    // pub expires_at: Option<String>,
    // pub side: Option<String>,
    // pub quantity: Option<i64>,
    // pub stop_price: Option<i64>,
    // pub limit_price: Option<i64>,
    // pub venue: Option<i64>,
    // pub estimated_price: Option<i64>,
    // pub notes: Option<String>,
    // pub idempotency: Option<String>,
    // pub charge: Option<i64>,
    // pub chargeable_at: Option<String>,
    // pub key_creation_id: Option<String>

}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderResults<RegulatoryInformation> {
    pub created_at: String,
    pub id: String,
    pub status: String,
    pub regulatory_information: Option<RegulatoryInformation>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegulatoryInformation {
    pub costs_entry: Option<i64>,
    pub costs_entry_pct : Option<String>,
    pub costs_running: Option<i64>,
    pub costs_running_pct: Option<String>,
    pub costs_product: Option<i64>,
    pub costs_product_pct: Option<String>,
    pub costs_exit: Option<i64>,
    pub costs_exit_pct: Option<String>,
    pub yield_reduction_year: Option<i64>,
    pub yield_reduction_year_pct: Option<String>,
    pub yield_reduction_year_following: Option<i64>,
    pub yield_reduction_year_following_pct: Option<String>,
    pub yield_reduction_year_exit: Option<i64>,
    pub yield_reduction_year_exit_pct: Option<String>,
    pub estimated_holding_duration_years: Option<String>,
    pub estimated_yield_reduction_total: Option<i64>,
    pub estimated_yield_reduction_total_pct: Option<String>,
    pub KIID: Option<String>,
    pub legal_disclaimer: Option<String>
} 



impl Client {
    /// Post and create a new order.
    pub fn post_an_order(
        &self,
        _body: OrderPlacing
    ) -> Result<OrderPlacingResponse, Error> {
        const PATH: &str = "account/withdrawals/";
        let resp = self.get::<OrderPlacingResponse>(PATH);
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
    fn test_placing_an_order() {
        dotenv::dotenv().unwrap();
        let api_key = env::var("LEMON_MARKET_TRADING_API_KEY").unwrap();
        let client = client::Client::paper_client(&api_key);
        let body = super::OrderPlacing {
            isin: "US0378331005".to_string(),
            expires_at: None,
            side: "buy".to_string(),
            quantity: 1,
            venue: None,
        };
        
        let resp = client.post_an_order(body).unwrap();
        dbg!(&resp);
        assert_eq!(resp.status, "ok");
    }
}