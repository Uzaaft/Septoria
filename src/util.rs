/// A macro that builds a query tuple by returning the var name and the value.
#[macro_export]
macro_rules! query_tuple {
    ($var: ident) => {
        (stringify!($var), $var)
    };
}

/// Private function
/// Builds a reqwest client with the given API key as a bearer auth token
pub(crate) fn build_reqwest_client(api_key: &str) -> reqwest::blocking::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );
    reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}
