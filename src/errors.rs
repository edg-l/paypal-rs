//! Errors created by this crate.
use crate::data::common::LinkDescription;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// A paypal api response error.
#[derive(Debug, Serialize, Deserialize)]
pub struct PaypalError {
    /// The error name.
    pub name: String,
    /// The error message.
    pub message: Option<String>,
    /// Paypal debug id
    pub debug_id: Option<String>,
    /// Error details
    pub details: Vec<HashMap<String, String>>,
    /// Only available on Identity errors
    pub error: Option<String>,
    /// Only available on Identity errors
    pub error_description: Option<String>,
    /// Links with more information about the error.
    pub links: Vec<LinkDescription>,
}

impl fmt::Display for PaypalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for PaypalError {}

/// A response error, it may be paypal related or an error related to the http request itself.
#[derive(Debug)]
pub enum ResponseError {
    /// A paypal api error.
    ApiError(PaypalError),
    /// A http error.
    HttpError(reqwest::Error),
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseError::ApiError(e) => write!(f, "{}", e),
            ResponseError::HttpError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for ResponseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ResponseError::ApiError(e) => Some(e),
            ResponseError::HttpError(e) => Some(e),
        }
    }
}

// Implemented so we can use ? directly on it.
impl From<PaypalError> for ResponseError {
    fn from(e: PaypalError) -> Self {
        ResponseError::ApiError(e)
    }
}

// Implemented so we can use ? directly on it.
impl From<reqwest::Error> for ResponseError {
    fn from(e: reqwest::Error) -> Self {
        ResponseError::HttpError(e)
    }
}

/// When a currency is invalid.
#[derive(Debug)]
pub struct InvalidCurrencyError(pub String);

impl fmt::Display for InvalidCurrencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} is not a valid currency", self.0)
    }
}

impl Error for InvalidCurrencyError {}

/// When a country is invalid.
#[derive(Debug)]
pub struct InvalidCountryError(pub String);

impl fmt::Display for InvalidCountryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} is not a valid country", self.0)
    }
}

impl Error for InvalidCountryError {}
