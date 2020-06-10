//! Errors created by this crate.
use std::collections::HashMap;
use std::fmt;
use std::error::Error;
use serde::{Deserialize, Serialize};

/// Represents a error HATEOAS link
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ErrorLink {
    /// The complete target URL.
    pub href: String,
     /// The link relation type, which serves as an ID for a link that unambiguously describes the semantics of the link.
    pub rel: String,
    /// The HTTP method required to make the related call.
    pub method: String,
}

/// A paypal api response error.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponseError {
    /// The error name.
    pub name: String,
    /// The error message.
    pub message: String,
    /// Paypal debug id
    pub debug_id: String,
    /// Error details
    pub details: Vec<HashMap<String, String>>,
    /// Only available on Identity errors
    pub error: Option<String>,
    /// Only available on Identity errors
    pub error_description: Option<String>,
    /// Links with more information about the error.
    pub links: Vec<ErrorLink>,
}

impl fmt::Display for ApiResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for ApiResponseError {}