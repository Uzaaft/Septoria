/// In general, an error will be returned in the following format.
/// Lemons docks: [Error Handling](https://docs.lemon.markets/error-handling)
use reqwest::{Error as ReqwestError, StatusCode};
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

    #[error("Test")]
    InternalServerError,

    #[error("Encountered an Json related error")]
    Json(#[from] JsonError),

    #[error("HTTP Error {0}")]
    Http(StatusCode),

    #[error("Service Unavailable")]
    ServiceUnavailable,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Bad Request")]
    BadRequest,

    #[error("{0}")]
    Str(String),
}
