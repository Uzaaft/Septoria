use std::fmt::Debug;

use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::{client::Client, error::Error};

pub mod account;
mod positions;

#[derive(Deserialize, Serialize, Debug)]
pub struct PaginationResponse<T> {
    pub time: String,
    pub status: String,
    pub mode: String,
    pub results: Option<Vec<T>>,
    pub previous: Option<String>,
    pub next: Option<String>,
    pub total: i64,
    pub page: i32,
    pub pages: i32,
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

    /// Private utility function to handle responses and errors
    // TODO: Actually use this in the code
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
    /// Private functions to handle query params
    pub fn get_query_string<Q: Serialize>(query: Q, query_vector: &mut Vec<String>) {
        if let Ok(query_string) = serde_urlencoded::to_string(&query) {
            query_vector.push(query_string);
        }
    }
}
