use serde::{Deserialize, Serialize};

use crate::api::{PaginationResponse, Response};
use crate::{client::Client, error::Error};
use chrono::prelude::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct InstrumentResponse {
    pub time: DateTime<Utc>,
    pub results: Option<Vec<InstrumentInfo>>,
    pub previous: String,
    pub next: String,
    pub total: i64,
    pub page: i64,
    pub pages: i64,
}


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
    pub isin: String,
    pub wkn: String,
    pub name: String,
    pub title: String,
    pub symbol: String,
    #[serde(rename = "type")]
    pub instrument_type: String,
    pub venues: Vec<InstrumentVenue>,
}

impl Client{
    /// Get a list of instruments.
    pub fn get_instruments(&self) -> Result<InstrumentResponse, Error> {
        const PATH: &str = "instruments/";
        let resp = self.get::<InstrumentResponse>(PATH);
        match resp {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }
}


#[cfg(test)]
mod tests{
    use std::env;
    use crate::*;
    #[test]
    fn test_get_instruments(){
        dotenv::dotenv().unwrap();
        let api_key = env::var("LEMON_MARKET_DATA_API_KEY").unwrap();
        let client = client::Client::data_client(&api_key);
        let instruments = client.get_instruments().unwrap();
        dbg!(instruments);
    }
}