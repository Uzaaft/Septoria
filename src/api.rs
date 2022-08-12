use std::fmt::Debug;
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize, Serializer};

use crate::{client::Client, error::Error};

pub mod account;

impl Client {
    /// Generic get request
    pub fn get<T: DeserializeOwned + Debug>(&self, path: &str) -> Result<T, Error> {
        let url = format!("{}/{}", self.base_url, path);
        let r = self.client.get(&url).send()?.json::<T>()?;
        Ok(r)
    }

    /// Generic get request with query parameters
    pub fn get_with_query<T: DeserializeOwned + Debug, Q: IntoIterator + Serialize>(
        &self,
        path: &str,
        query: Q,
    ) -> Result<T, Error> {
        let url = format!("{}/{}", self.base_url, path);
        let r = self.client.get(&url).query(&query).send()?;
        if r.status() == StatusCode::OK {
            let r = r;
            let r = r.json::<T>()?; // Panic happens here
            Ok(r)
        } else {
            Err(Error::Str("error".to_string()))
        }
    }

    /// Generic post request
    pub fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: B) -> Result<T, Error> {
        let url = format!("{}/{}", self.base_url, path);
        let body_string = serde_json::to_string(&body)?;
        let r = self
            .client
            .post(&url)
            .body(body_string)
            .send()?;
            let json = r.json::<T>()?;
        Ok(json)
    }

    /// Generic delete request
    pub fn delete<T: DeserializeOwned, PP: Serialize>(
        &self,
        path: &str,
        path_param: &str,
    ) -> Result<T, Error> {
        let url = format!("{}/{}/{}", self.base_url, path, path_param);
        let r = self.client.delete(&url).send()?.json::<T>()?;
        Ok(r)
    }

    /// Private utility function to handle responses and errors
    // TODO: Actually use this in the code
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
