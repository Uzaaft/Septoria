use serde::{Deserialize, Serialize};
use chrono::prelude::*;


use crate::{client::Client, error::Error};

#[derive(Serialize, Deserialize, Debug)]
// The struct for placing an order - body of the post request
pub struct OrderPlacing {
    pub isin: String,
    pub expires_at: Option<String>,
    pub side: OrderType, // "buy" or "sell"
    pub quantity: i64,
    pub venue: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OrderType {
    buy,
    sell
}

#[derive(Serialize, Deserialize, Debug)]
// The struct for placing an order - the response of the request
pub struct OrderPlacingResponse<T> {
    pub time: String,
    pub status: String,
    pub mode: String,
    pub results: Option<T>,
}

#[derive(Serialize, Deserialize, Debug)]
// The struct for placing an order - the results response of the request
pub struct OrderResults {
    pub created_at: DateTime<Utc>,
    pub id: String,
    pub status: String,
    pub regulatory_information: Option<RegulatoryInformation>,
    pub isin: Option<String>,
    pub expires_at: Option<String>,
    pub side: Option<OrderType>,
    pub quantity: Option<i64>,
    pub stop_price: Option<String>,
    pub limit_price: Option<String>,
    pub venue: Option<String>,
    pub estimated_price: Option<i64>,
    pub notes: Option<String>,
    pub idempotency: Option<String>,
    pub charge: Option<i64>,
    pub chargeable_at: Option<String>,
    pub key_creation_id: Option<String>,
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
    ) -> Result<OrderPlacingResponse<OrderResults>, Error> {
        const PATH: &str = "orders/";
        let resp = self.post::<OrderPlacingResponse<OrderResults>, _>(PATH, _body);
        match resp {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }
}


#[cfg(test)]
mod test {
    use std::env;
    use chrono::prelude::*;


    use crate::*;
    #[test]
    fn test_placing_an_order() {
        dotenv::dotenv().unwrap();
        let local: DateTime<Local> = Local::now();
        let api_key = env::var("LEMON_MARKET_TRADING_API_KEY").unwrap();
        let client = client::Client::paper_client(&api_key);
        let body = super::OrderPlacing {
            isin: "US0378331005".to_string(),
            expires_at: Some(local.format("%Y-%m-%d").to_string()), 
            side: super::OrderType::buy,
            quantity: 1,
            venue: Some("XMUN".to_string()),
        };
        dbg!(&body);
        
        let resp = client.post_an_order(body).unwrap();
        dbg!(&resp);
        assert_eq!(resp.status, "ok");
    }
}