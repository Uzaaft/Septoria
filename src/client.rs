static PAPER_ENDPOINT: &str = "https://paper-trading.lemon.markets/v1/";
static DATA_ENDPOINT: &str = "https://data.lemon.markets/v1/";
static LIVE_ENDPOINT: &str = "https://trading.lemon.markets/v1/";

#[derive(Debug)]
pub struct Client {
    pub api_key: String,
    pub base_url: String,
    client: reqwest::Client,
}

impl Client {
    // Internal function to create new client
    fn new(api_key: &str, base_url: String) -> Self {
        let client = build_reqwest_client(&api_key);
        Client {
            api_key: api_key.to_owned(),
            base_url,
            client,
        }
    }

    pub fn paper_client(api_key: &str) -> Self {
        Client::new(api_key, PAPER_ENDPOINT.to_string())
    }
    pub fn data_client(api_key: &str) -> Self {
        Client::new(api_key, DATA_ENDPOINT.to_string())
    }
    pub fn live_client(api_key: &str) -> Self {
        Client::new(api_key, LIVE_ENDPOINT.to_string())
    }
}

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

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_client() {
        let client = Client::paper_client("12345".to_string());

        assert_eq!(client.base_url, PAPER_ENDPOINT);
    }
}
