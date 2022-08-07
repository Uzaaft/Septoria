use serde::de::DeserializeOwned;

use crate::client::Client;

pub mod account;

impl Client {
    /// Methd for a get request to given path.
    fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, reqwest::Error> {
        let url = format!("{}/{}", self.base_url, path);
        let resp = self.client.get(&url).send()?.json::<T>()?;
        Ok(resp)
    }
}
