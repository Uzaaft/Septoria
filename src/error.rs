/// In general, an error will be returned in the following format.
/// Lemons docks: [Error Handling](https://docs.lemon.markets/error-handling)
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Encountered an Reqwest related error")]
    Reqwest(
        #[from]
        #[source]
        ReqwestError,
    ),

    #[error("Encountered an Json related error")]
    Json(
        #[from]
        #[source]
        JsonError,
    ),
}
