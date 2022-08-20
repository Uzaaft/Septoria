use crate::api::Requests;
use crate::error::Error;
use crate::util::build_reqwest_client;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

/// Paper endpoint url
static PAPER_ENDPOINT: &str = "https://paper-trading.lemon.markets/v1/";
/// Money endpoint url
static MONEY_ENDPOINT: &str = "https://trading.lemon.markets/v1/";

#[derive(Debug)]
/// The client for the Lemon API.
pub struct TradingClient {
    /// The API key.
    pub api_key: String,
    /// The base url for the API
    pub base_url: Url,
    /// Internal client used for all requests.
    pub(crate) client: reqwest::blocking::Client,
}

/// API methods for the Client
impl Requests for TradingClient {
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
    fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: B) -> Result<T, Error> {
        let url = format!("{}/{}", self.base_url, path);
        let body_string = serde_json::to_string(&body)?;
        let r = self.client.post(&url).body(body_string).send()?;
        dbg!(&r);
        let json = self.response_handler(r)?;
        Ok(json)
    }
    /// Generic delete request
    fn delete<T: DeserializeOwned>(&self, path: &str, path_param: &str) -> Result<T, Error> {
        let url = format!("{}/{}/{}", self.base_url, path, path_param);
        let r = self.client.delete(&url).send()?;
        // .json::<T>()?;
        let json = self.response_handler::<T>(r)?;
        Ok(json)
    }
}

impl TradingClient {
    /// Create a new TradingClient
    pub fn new(api_key: String, _endpoint: &str) -> Self {
        let base_url = Url::parse(PAPER_ENDPOINT).unwrap();
        let client = build_reqwest_client(&api_key);
        Self {
            api_key,
            base_url,
            client,
        }
    }

    /// Create a new client for paper trading with the given API key.
    pub fn paper_client(api_key: &str) -> Self {
        TradingClient::new(api_key.to_string(), PAPER_ENDPOINT)
    }

    /// Create a new client for live trading with the given API key.
    pub fn live_client(api_key: String) -> Self {
        TradingClient::new(api_key, MONEY_ENDPOINT)
    }
}
