#[cfg(test)]
mod tests;

pub mod errors;

use serde::Deserialize;
use std::time::{Duration, Instant};

pub const LIVE_ENDPOINT: &str = "https://api.paypal.com";
pub const SANDBOX_ENDPOINT: &str = "https://api.sandbox.paypal.com";

/// Represents the access token returned by the oauth2 authentication.
///
/// https://developer.paypal.com/docs/api/get-an-access-token-postman/
#[derive(Debug, Deserialize)]
pub struct AccessToken {
    pub scope: String,
    pub access_token: String,
    pub token_type: String,
    pub app_id: String,
    pub expires_in: u64,
    pub nonce: String,
}

#[derive(Debug)]
pub struct Auth<'a> {
    pub client_id: &'a str,
    pub secret: &'a str,
    pub access_token: Option<AccessToken>,
    pub expires: Option<(Instant, Duration)>,
}

/// Represents a client used to interact with the paypal api.
#[derive(Debug)]
pub struct Client<'a> {
    pub client: reqwest::Client,
    pub sandbox: bool,
    pub auth: Auth<'a>,
}

impl<'a> Client<'a> {
    /// Returns a new client, you must get_access_token afterwards to interact with the api.
    pub fn new(client_id: &'a str, secret: &'a str, sandbox: bool) -> Client<'a> {
        Client {
            client: reqwest::Client::new(),
            sandbox,
            auth: Auth {
                client_id,
                secret,
                access_token: None,
                expires: None,
            },
        }
    }

    fn endpoint(&self) -> &str {
        if self.sandbox {
            SANDBOX_ENDPOINT
        } else {
            LIVE_ENDPOINT
        }
    }

    /// Gets a access token used in all the api calls.
    pub async fn get_access_token(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .client
            .post(format!("{}/v1/oauth2/token", self.endpoint()).as_str())
            .basic_auth(self.auth.client_id, Some(self.auth.secret))
            .header("Content-Type", "x-www-form-urlencoded")
            .header("Accept", "application/json")
            .body("grant_type=client_credentials")
            .send()
            .await?;

        if res.status().is_success() {
            let token = res.json::<AccessToken>().await?;
            self.auth.expires = Some((Instant::now(), Duration::new(token.expires_in, 0)));
            self.auth.access_token = Some(token);
            println!("{:#?}", self.auth);
        } else {
            println!("status = {:#?}", res.status());
            println!("res = {:#?}", res);
            return Err(Box::new(errors::GetAccessTokenError));
        }

        Ok(())
    }

    /// Checks if the access token expired.
    pub fn access_token_expired(&self) -> bool {
        if let Some(expires) = self.auth.expires {
            expires.0.elapsed() >= expires.1
        } else {
            true
        }
    }
}
