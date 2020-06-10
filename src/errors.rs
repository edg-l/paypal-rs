//! Errors created by this crate.
use std::collections::HashMap;
use std::fmt;
use std::error::Error;
use serde::{Deserialize, Serialize};

/// Represents a error HATEOAS link
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ErrorLink {
    href: String,
    rel: String,
    method: String,
}

/// A paypal api response error.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponseError {
    /// The error name.
    name: String,
    /// The error message.
    message: String,
    /// Paypal debug id
    debug_id: String,
    /// Error details
    details: Vec<HashMap<String, String>>,
    /// Only available on Identity errors
    error: Option<String>,
    /// Only available on Identity errors
    error_description: Option<String>,
    /// Links with more information about the error.
    links: Vec<ErrorLink>,
}

impl fmt::Display for ApiResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for ApiResponseError {}