use serde::de::DeserializeOwned;

use crate::{client::Client, error::Error};

pub mod account;

impl Client {
    fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let url = format!("{}/{}", self.base_url, path);
        let r = self.client.get(&url).send()?.json::<T>()?;
        Ok(r)
    }
}
