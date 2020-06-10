//! Errors created by this crate.

use thiserror::Error;

/// A enum that represents the possible errors.
#[derive(Debug, Error)]
pub enum Errors {
    /// A error used when a api call fails.
    #[error("failure when calling the paypal api")]
    ApiCallFailure(String),
}