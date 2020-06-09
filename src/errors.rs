use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error("failed to get access token")]
    GetAccessTokenFailure,
    #[error("failure when calling the paypal api")]
    ApiCallFailure,
}