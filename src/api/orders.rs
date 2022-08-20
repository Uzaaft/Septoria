use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api::{GenericResponse, Requests};
use crate::client::TradingClient;
use crate::{api::Response, error::Error};

#[derive(Serialize, Deserialize, Debug)]
// The struct for placing an order - body of the post request
pub struct OrderPlacing {
    pub isin: String,
    pub expires_at: Option<String>,
    pub side: OrderType,
    // "buy" or "sell"
    pub quantity: i64,
    pub venue: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Buy,
    Sell,
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
    /// Timestamp for when you created your order
    pub created_at: DateTime<Utc>,
    /// Unique Identification number for your order
    pub id: String,
    /// The status of your order
    pub status: String,
    /// Regulatory information about the order
    pub regulatory_information: Option<RegulatoryInformation>,
    /// Where the order was placed
    pub isin: Option<String>,
    /// Expiry date of the order
    pub expires_at: Option<String>,
    /// The type of order you placed (buy or sell)
    pub side: Option<OrderType>,
    /// The quantity of the order you placed
    pub quantity: Option<i64>,
    /// The stop_price of the order you placed
    pub stop_price: Option<String>,
    /// The limit_price of the order you placed
    pub limit_price: Option<String>,
    /// The venue of the order you placed
    pub venue: Option<String>,
    /// The estimated price of the order you placed
    pub estimated_price: Option<i64>,
    /// Notes about your order
    pub notes: Option<String>,
    /// Idempotency for the request
    pub idempotency: Option<String>,
    /// Charges for the order request
    pub charge: Option<i64>,
    // Charges for the order request
    pub chargeable_at: Option<String>,
    /// Key creation identifier for the order request
    pub key_creation_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegulatoryInformation {
    pub costs_entry: Option<i64>,
    pub costs_entry_pct: Option<String>,
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
    #[serde(rename = "KIID")]
    pub kiid: Option<String>,
    pub legal_disclaimer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivateOrder {
    pub id: String,
    pub pin: i64,
}

impl TradingClient {
    /// Post and create a new order.
    pub fn post_order(&self, _body: OrderPlacing) -> Result<GenericResponse<OrderResults>, Error> {
        const PATH: &str = "orders/";
        let resp = self.post::<GenericResponse<OrderResults>, _>(PATH, _body);
        match resp {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }

    /// Activate an order by id
    pub fn activate_order(&self, pin: i64, order_id: &str) -> Result<Response, Error> {
        let url = format!("orders/{order_id}/activate");
        let resp = self.post::<Response, ActivateOrder>(
            url.as_str(),
            ActivateOrder {
                id: order_id.to_string(),
                pin,
            },
        );
        match resp {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }

    /// Delete an order by id
    pub fn delete_order(&self, order_id: &str) -> Result<Response, Error> {
        const PATH: &str = "orders/{order_id}/";
        let resp = self.delete::<Response>(PATH, order_id);
        match resp {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::prelude::*;
    use std::env;

    use crate::*;

    #[test]
    fn test_placing_and_activating_an_order() {
        dotenv::dotenv().unwrap();
        let local: DateTime<Local> = Local::now();
        let api_key = env::var("LEMON_MARKET_TRADING_API_KEY").unwrap();
        let client = client::TradingClient::paper_client(&api_key);
        let body = super::OrderPlacing {
            isin: "US0378331005".to_string(),
            expires_at: Some(local.format("%Y-%m-%d").to_string()),
            side: super::OrderType::Buy,
            quantity: 1,
            venue: Some("XMUN".to_string()),
        };
        let resp = client.post_order(body).unwrap();
        dbg!(&resp);
        assert_eq!(resp.status, "ok");
        let resp = client
            .activate_order(1234, resp.results.unwrap().id.as_str())
            .unwrap();
        dbg!(&resp);
        assert_eq!(resp.status, "ok");
    }
}
