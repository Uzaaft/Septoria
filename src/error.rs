/// In general, an error will be returned in the following format.
/// Lemons docks: [Error Handling](https://docs.lemon.markets/error-handling)
use reqwest::{Error as ReqwestError, StatusCode};
use serde_json::Error as JsonError;
use thiserror::Error;


/// Error type for the library
#[derive(Debug, Error)]
pub enum Error {
    #[error("Encountered an Reqwest related error")]
    /// Error type for Reqwest errors
    Reqwest(
        #[from]
        #[source]
        ReqwestError,
    ),

    /// Error type for Json errors
    #[error("Encountered an Json related error")]
    Json(#[from] JsonError),

    /// Error type for StatusCode errors
    #[error("HTTP Error {0}")]
    Http(StatusCode),

    /// Error type for other errors
    #[error("{0}")]
    Str(String),
}
