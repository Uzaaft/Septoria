use std::fmt::Debug;
use reqwest::{StatusCode, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::api::Requests;
use crate::error::Error;

use crate::util::build_reqwest_client;

static DATA_ENDPOINT: &str = "https://data.lemon.markets/v1/";

#[derive(Debug)]
/// The data client for the Lemon API.
pub struct DataClient {
    /// The API key.
    pub api_key: String,
    /// The base url for the API
    pub base_url: Url,
    /// Internal client used for all requests.
    pub(crate) client: reqwest::blocking::Client,
}

impl DataClient{
    /// Create a new data client.
    pub fn new(api_key: String) -> Self {
        let client = build_reqwest_client(&api_key);
        Self {
            api_key,
            base_url: Url::parse(DATA_ENDPOINT).unwrap(),
            client,
        }
    }
}

impl Requests for DataClient {
    /// Generic get request
    fn get<T: DeserializeOwned + Debug>(&self, path: &str) -> Result<T, Error> {
        let url = format!("{}/{}", self.base_url, path);
        let r = self.client.get(&url).send()?;
        let json = self.response_handler(r)?;
        Ok(json)
    }
    /// Generic get request with query parameters
    fn get_with_query<T: DeserializeOwned + Debug, Q: IntoIterator + Serialize>(
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
    fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: B,
    ) -> Result<T, Error> {
        let url = format!("{}/{}", self.base_url, path);
        let body_string = serde_json::to_string(&body)?;
        let r = self.client.post(&url).body(body_string).send()?;
        dbg!(&r);
        let json = self.response_handler(r)?;
        Ok(json)
    }
    /// Generic delete request
    fn delete<T: DeserializeOwned>(
        &self,
        path: &str,
        path_param: &str,
    ) -> Result<T, Error> {
        let url = format!("{}/{}/{}", self.base_url, path, path_param);
        let r = self.client.delete(&url).send()?;
        let json = self.response_handler::<T>(r)?;
        Ok(json)
    }
}