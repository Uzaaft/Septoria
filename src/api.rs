//! Generic functions and structs for the API
//!
//! This module contains the generic functions and structs for the API.
//! These are used by the `Client`, and public for the whole crate

use chrono::prelude::*;
use std::fmt::Debug;

use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::error::{Error, LemonError};

/// Module for interacting with the account related endpoints
mod market_data;
mod orders;
mod trading;

/// Generic struct for Endpoints that returns pagination information alongside data
///
/// Properties:
///
/// * `time`: The time the request was made.
/// * `status`: The status of the request.
/// * `mode`: The mode of the request.
/// * `results`: The actual results of the query.
/// * `previous`: The URL of the previous page of results.
/// * `next`: The URL for the next page of results.
/// * `total`: The total number of results available.
/// * `page`: The current page number
/// * `pages`: The total number of pages in the response.
#[derive(Deserialize, Serialize, Debug)]
pub struct PaginationResponse<T> {
    /// The time the request was made.
    pub time: DateTime<Utc>,
    /// The status of the request.
    pub status: Option<String>,
    /// The mode of the request. Can be paper, live, or market_data
    pub mode: Option<Mode>,
    /// The actual results of the query. Depends upon the given generics
    pub results: Option<Vec<T>>,
    /// The URL of the previous page of results.
    pub previous: Option<String>,
    /// The URL for the next page of results.
    pub next: Option<String>,
    /// The total number of results available.
    pub total: i64,
    /// The current page number
    pub page: i64,
    /// The total number of pages in the response.
    pub pages: i64,
}

/// General response struct
#[derive(Deserialize, Debug)]
pub struct Response {
    /// Timestamp of your request
    pub time: String,
    /// Environment the request was placed in: "paper" or "money"
    // TODO: Make this an enum
    pub mode: Mode,
    /// Status of the request. Returns 'ok' if successful
    pub status: String,
}

/// Generic response struct
#[derive(Deserialize, Debug)]
pub struct GenericResponse<T> {
    /// Timestamp of your request
    pub time: DateTime<Utc>,
    /// Environment the request was placed in: "paper" or "money"
    // TODO: Make this an enum
    pub mode: Mode,
    /// Status of the request.
    pub status: String,
    /// The actual results of the query. Depends upon the given generics
    pub results: Option<T>,
}

/// Trading mode
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Paper trading mode
    Paper,
    /// Live trading mode
    Live,
    /// Market data mode
    MarketData,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Mode::Paper => write!(f, "paper"),
            Mode::Live => write!(f, "live"),
            Mode::MarketData => write!(f, "market_data"),
        }
    }
}

/// Traits with API methods that are common across all calls
pub(crate) trait Requests {
    /// Generic get request
    fn get<T: DeserializeOwned + Debug>(&self, path: &str) -> Result<T, Error>;
    /// Generic get request with query parameters
    fn get_with_query<T: DeserializeOwned + Debug, Q: IntoIterator + Serialize>(
        &self,
        path: &str,
        query: Q,
    ) -> Result<T, Error>;

    /// Generic post request
    fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: B) -> Result<T, Error>;

    /// Generic delete request
    fn delete<T: DeserializeOwned>(&self, path: &str, path_param: &str) -> Result<T, Error>;

    /// Crate wide function to handle responses and errors
    fn response_handler<T: DeserializeOwned>(
        &self,
        response: reqwest::blocking::Response,
    ) -> Result<T, Error> {
        match response.status() {
            StatusCode::OK => Ok(response.json::<T>()?),
            _s => {
                let message = response.json::<LemonError>()?;
                Err(Error::Str(message.to_string()))
            }
        }
    }
    /// Crate wide function to handle query params
    fn get_query_string<Q: Serialize>(query: Q, query_vector: &mut Vec<String>) {
        if let Ok(query_string) = serde_urlencoded::to_string(&query) {
            query_vector.push(query_string);
        }
    }
}
