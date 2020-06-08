use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct GetAccessTokenError;

impl fmt::Display for GetAccessTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error getting access token")
    }
}

impl Error for GetAccessTokenError {}