/// In general, an error will be returned in the following format.
/// Lemons docks: [Error Handling](https://docs.lemon.markets/error-handling)
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use reqwest::Error as ReqwestError;
use serde::Deserialize;
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
}

#[derive(Debug, Deserialize, Error)]
#[error("")]
pub struct ApiError {
    #[serde(with = "ts_milliseconds")]
    pub time: DateTime<Utc>,
    #[serde(rename = "error_code")]
    pub status: ErrorCode,
    #[serde(rename = "error_message")]
    pub message: String,
}

// Not sure if this is the best way to map all the different
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    Unauthorized,
    TokenInvalid,
    RateLimitExceeded,
    InternalError,
    OrderIdempotencyViolation,
    PinMissing,
    PinNotSet,
    PinInvalid,
    WithdrawInsufficientFunds,
    WithdrawLimitExceeded,
    WithdrawRequestLimitExceeded,
    ForbiddenInCurrentState,
    PlanNotAllowed,
}
