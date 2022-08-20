//! A crate for interacting with the Lemon market_data API

#![deny(missing_docs)]
pub mod api;
/// API client for the Lemon market_data API
pub mod client;
/// Error type for the Lemon market_data API
pub mod error;
/// Module for utilities
mod util;
