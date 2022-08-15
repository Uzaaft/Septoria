//! Generic functions and structs for the API
//!
//! This module contains the generic functions and structs for the API.
//! These are used by the `Client`, and public for the whole crate

use std::fmt::Debug;

use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::{client::Client, error::Error};

/// Module for interacting with the account related endpoints
pub mod account;
mod positions;
mod orders;

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
    pub time: String,
    /// The status of the request.
    pub status: String,
    /// The mode of the request. Can be paper, live, or market
    pub mode: String,
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
    pub mode: String,
    /// Status of the request. Returns 'ok' if successful
    pub status: String,
}

/// Generic response struct
#[derive(Deserialize, Debug)]
pub(crate) struct GenericResponse<T> {
    /// Timestamp of your request
    pub time: String,
    /// Environment the request was placed in: "paper" or "money"
    // TODO: Make this an enum
    pub mode: String,
    /// Status of the request.
    pub status: T,
}

impl Client {
    /// Generic get request
    pub(crate) fn get<T: DeserializeOwned + Debug>(&self, path: &str) -> Result<T, Error> {
        let url = format!("{}/{}", self.base_url, path);
        let r = self.client.get(&url).send()?;
        let json = self.response_handler(r)?;
        Ok(json)
    }

    /// Generic get request with query parameters
    pub(crate) fn get_with_query<T: DeserializeOwned + Debug, Q: IntoIterator + Serialize>(
        &self,
        path: &str,
        query: Q,
    ) -> Result<T, Error> {
        let url = format!("{}/{}", self.base_url, path);
        let r = self.client.get(&url).query(&query).send()?;
        let json = self.response_handler(r)?;
        Ok(json)
    }

    /// Generic post request
    pub(crate) fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: B,
    ) -> Result<T, Error> {
        let url = format!("{}/{}", self.base_url, path);
        let body_string = serde_json::to_string(&body)?;
        let r = self.client.post(&url).body(body_string).send()?;
        let json = self.response_handler(r)?;
        Ok(json)
    }

    /// Generic delete request
    pub(crate) fn delete<T: DeserializeOwned, PP: Serialize>(
        &self,
        path: &str,
        path_param: &str,
    ) -> Result<T, Error> {
        let url = format!("{}/{}/{}", self.base_url, path, path_param);
        let r = self.client.delete(&url).send()?;
        // .json::<T>()?;
        let json = self.response_handler::<T>(r)?;
        Ok(json)
    }

    /// Crate wide function to handle responses and errors
    pub(crate) fn response_handler<T: DeserializeOwned>(
        &self,
        response: reqwest::blocking::Response,
    ) -> Result<T, Error> {
        match response.status() {
            StatusCode::OK => Ok(response.json::<T>()?),
            s => {
                let message = s.to_string();
                Err(Error::Str(message))
            }
        }
    }
    /// Crate wide function to handle query params
    pub(crate) fn get_query_string<Q: Serialize>(query: Q, query_vector: &mut Vec<String>) {
        if let Ok(query_string) = serde_urlencoded::to_string(&query) {
            query_vector.push(query_string);
        }
    }
}
