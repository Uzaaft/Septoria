use crate::api::Mode;
use chrono::prelude::*;
/// In general, an error will be returned in the following format.
/// Lemons docks: [Error Handling](https://docs.lemon.markets/error-handling)
use reqwest::{Error as ReqwestError, StatusCode};
use serde::{Deserialize, Serialize};
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

/// Error type for the Lemon API
#[derive(Deserialize)]
pub struct LemonError {
    /// The time that the error occurred
    time: DateTime<Utc>,
    /// API mode.
    mode: Mode,
    /// Status Code of the error
    status: String,
    /// Lemon API error code
    error_code: ErrorCode,
    /// Error message
    error_message: String,
}

/// Error codes for the Lemon API
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    /// The API key is not provided in the HTTP header,
    /// cannot be decoded by the backend,
    /// or the API Key does not exist.
    Unauthorized,
    /// The API key is revoked or user is deleted/suspended.
    TokenInvalid,
    /// The API key has exceeded its rate limit.
    /// Please respect the value of the Retry-After
    /// header before performing another request.
    RateLimitExceeded,
    // TODO: Add [xxx_not_found]
    /// An error occurred in the backend.
    /// This is not your fault.
    /// We will investigate this.
    InternalError,
    /// Same idempotency has been used within current 7 day period.
    OrderIdempotencyViolation,
    /// Cannot withdraw money because the PIN is not provided in the request (money only).
    PinMissing,
    /// Cannot withdraw money because the PIN is not set (money only)
    PinNotSet,
    /// Cannot withdraw money because the pin verification failed
    PinInvalid,
    /// Cannot withdraw money because there are insufficient funds on the account
    WithdrawInsufficientFunds,
    /// Cannot withdraw money because the daily payout limit is exceeded
    WithdrawLimitExceeded,
    /// Cannot withdraw money because the maximal daily request limit is exceeded
    WithdrawRequestLimitExceeded,
    /// Cannot update address_country when trading is enabled or user is onboarded
    ForbiddenInCurrentState,
    /// Cannot update trading plan to basic/pro
    PlanNotAllowed,
    ///    insufficient instrument holdings [on order sell]
    InsufficientHoldings,
    ///    cannot place order if one expires before market opens again
    OrderExpirationDateInvalid,
    ///    cannot place/activate buy order if estimated total price is greater than 25k Euro
    OrderTotalPriceLimitExceeded,
    ///cannot place order in ALLDAY for MONEY env
    ForbiddenForVenue,
    ///    cannot place order if trading is not enabled
    TradingDisabled,
    ///maximum daily amount of orders reached
    OrderLimitExceeded,
    ///failed to place order if instrument is not tradable
    InstrumentNotTradable,
    ///cannot place/activate buy order because of insufficient account funds
    AccountInsufficientFunds,
    ///cannot place/activate order because trading is blocked globally
    TradingBlocked,
    ///cannot activate order if its status != inactive
    OrderNotInactive,
    /// cannot delete order if its not in cancelling/cancelled/expired/executed/rejected state
    OrderNotTerminated,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for LemonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "time: {}, mode: {}, status: {}, error_code: {}, error_message: {}",
            self.time, self.mode, self.status, self.error_code, self.error_message
        )
    }
}
