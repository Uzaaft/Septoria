/// Paper endpoint url
static PAPER_ENDPOINT: &str = "https://paper-trading.lemon.markets/v1/";
/// Data endpoint url
static DATA_ENDPOINT: &str = "https://data.lemon.markets/v1/";
/// Trading endpoint url
static LIVE_ENDPOINT: &str = "https://trading.lemon.markets/v1/";

use reqwest::Url;
#[derive(Debug)]
/// The client for the Lemon API.
pub struct Client {
    /// The API key.
    pub api_key: String,
    /// The base url for the API
    pub base_url: Url,
    /// private reqwest client
    client: reqwest::Client,
}

impl Client {
    // Internal function to create new client
    fn new(api_key: &str, base_url: String) -> Self {
        let client = build_reqwest_client(api_key);
        Client {
            api_key: api_key.to_owned(),
            base_url: base_url.parse::<Url>().unwrap(), // Usage of unwrap here is safe, as the url is always valid
            client,
        }
    }

    /// Create a new client for paper trading with the given API key.
    pub fn paper_client(api_key: &str) -> Self {
        Client::new(api_key, PAPER_ENDPOINT.to_string())
    }

    /// Create a new client for fetching data with the given API key.    
    pub fn data_client(api_key: &str) -> Self {
        Client::new(api_key, DATA_ENDPOINT.to_string())
    }

    /// Create a new client for live trading with the given API key.
    pub fn live_client(api_key: &str) -> Self {
        Client::new(api_key, LIVE_ENDPOINT.to_string())
    }
}

/// Private function
/// Builds a reqwest client with the given API key as a bearer auth token
fn build_reqwest_client(api_key: &str) -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );
    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}
