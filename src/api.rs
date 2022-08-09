use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use crate::{client::Client, error::Error};

pub mod account;

impl Client {
    /// Generic get request
    fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let url = format!("{}/{}", self.base_url, path);
        let r = self.client.get(&url).send()?.json::<T>()?;
        Ok(r)
    }

    /// Generic post request
    fn post<T: DeserializeOwned>(&self, path: &str, body: String) -> Result<T, Error> {
        let url = format!("{}/{}", self.base_url, path);
        let r = self.client.post(&url).body(body).send()?.json::<T>()?;
        Ok(r)
    }

    fn response_handler<T: DeserializeOwned>(
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
}
