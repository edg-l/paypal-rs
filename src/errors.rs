//! Errors created by this crate.
use std::collections::HashMap;
use std::fmt;
use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::common::LinkDescription;

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
    pub links: Vec<LinkDescription>,
}

impl fmt::Display for ApiResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for ApiResponseError {}