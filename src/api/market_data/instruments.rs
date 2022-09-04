use serde::{Deserialize, Serialize};

use crate::api::{Requests, PaginationResponse};
use crate::client::TradingClient;
use crate::{data_client::DataClient, error::Error, query_tuple};
use chrono::prelude::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct InstrumentVenue {
    pub name: String,
    pub title: String,
    pub mic: String,
    pub is_open: bool,
    pub tradable: bool,
    pub currency: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstrumentInfo {
    pub isin: Option<String>,
    pub wkn: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub symbol: Option<String>,
    #[serde(rename = "type")]
    pub instrument_type: Option<String>,
    pub venues: Option<Vec<InstrumentVenue>>,
}

impl DataClient {
    /// Get a list of instruments.
    pub fn get_instruments(
        &self,
        isin: Option<String>,
        search: Option<String>,
        instrument_type: Option<String>,
    ) -> Result<PaginationResponse<InstrumentInfo>, Error> {
        const PATH: &str = "instruments/";

        // Build query
        let mut query: Vec<String> = vec![];
        TradingClient::get_query_string(query_tuple!(isin), &mut query);
        TradingClient::get_query_string(query_tuple!(search), &mut query);
        TradingClient::get_query_string(query_tuple!(instrument_type), &mut query);

        let resp = self.get_with_query::<PaginationResponse<InstrumentInfo>, Vec<String>>(PATH, query);
        match resp {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_client::DataClient;

    use std::env;

    #[test]
    fn test_get_instruments() {
        dotenv::dotenv().unwrap();
        let api_key = env::var("LEMON_MARKET_DATA_API_KEY").unwrap();
        let client = DataClient::new(api_key);
        let stock_name = "Aker";
        let instruments = client
            .get_instruments(None, Some(stock_name.to_string()), None)
            .unwrap();
    }
}
