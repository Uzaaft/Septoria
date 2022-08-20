use serde::{Deserialize, Serialize};

use crate::api::{PaginationResponse, Response};
use crate::{client::Client, error::Error};
use chrono::prelude::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct OpeningHours {
    pub start: String,
    pub end: String,
    pub timezone: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VenueData {
    pub name: String,
    pub title: String,
    pub mic: String,
    pub is_open: bool,
    pub opening_hours: OpeningHours,
    pub opening_days: Vec<NaiveDate>,
}
type VenueDataPagination = PaginationResponse<VenueData>;

impl Client{
    /// Get a list of venues.
    pub fn get_venues(&self) -> Result<VenueDataPagination, Error> {
        const PATH: &str = "venues/";
        let resp = self.get::<VenueDataPagination>(PATH);
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
    fn test_get_venues(){
        dotenv::dotenv().unwrap();
        let api_key = env::var("LEMON_MARKET_DATA_API_KEY").unwrap();
        let client = client::Client::data_client(&api_key);
        let venues = client.get_venues().unwrap();
        dbg!(venues);
    }
}