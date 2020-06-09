#[cfg(test)]
mod tests;

extern crate chrono;

pub mod errors;
pub mod orders;

use serde::{Serialize, Deserialize};
use std::time::{Duration, Instant};
use reqwest::header::HeaderMap;
use reqwest::header;

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
pub struct Auth {
    pub client_id: String,
    pub secret: String,
    pub access_token: Option<AccessToken>,
    pub expires: Option<(Instant, Duration)>,
}

/// Represents a client used to interact with the paypal api.
#[derive(Debug)]
pub struct Client {
    pub client: reqwest::Client,
    pub sandbox: bool,
    pub auth: Auth,
}

/// Represents the query used in most GET api requests.
///
/// Reference: https://developer.paypal.com/docs/api/reference/api-requests/#query-parameters
///
/// Note: You can avoid most fields by the Default impl like so:
/// ```
/// let query = Query { count: Some(40), ..Default::default() };
/// ```
#[derive(Debug, Default)]
pub struct Query {
    pub count: Option<i32>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub total_count_required: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub start_id: Option<String>,
    pub start_index: Option<i32>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug)]
pub enum Prefer {
    Minimal,
    Representation,
}

/// Represents the optional header values used on paypal requests.
///
/// https://developer.paypal.com/docs/api/reference/api-requests/#paypal-auth-assertion
#[derive(Debug, Default)]
pub struct HeaderParams {
    pub  merchant_payer_id: Option<String>,
    pub client_metadata_id: Option<String>,
    pub partner_attribution_id: Option<String>,
    pub request_id: Option<String>,
    pub prefer: Option<Prefer>,
    pub content_type: Option<String>,
}

#[derive(Debug, Serialize)]
struct AuthAssertionClaims {
    pub iss: String,
    pub payer_id: String,
}

impl Client {
    /// Returns a new client, you must get_access_token afterwards to interact with the api.
    ///
    /// Example:
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenv().ok();
    ///     let clientid = env::var("PAYPAL_CLIENTID").unwrap();
    ///     let secret = env::var("PAYPAL_SECRET").unwrap();
    ///
    ///     let mut client = Client::new(
    ///         clientid.as_str(),
    ///         secret.as_str(),
    ///         true,
    ///     );
    ///
    ///     client.get_access_token().await.unwrap();
    ///     println!("{:#?}", client);
    /// }
    /// ```
    pub fn new<S: Into<String>>(client_id: S, secret: S, sandbox: bool) -> Client {
        Client {
            client: reqwest::Client::new(),
            sandbox,
            auth: Auth {
                client_id: client_id.into(),
                secret: secret.into(),
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

    /// Sets up the request headers as required on https://developer.paypal.com/docs/api/reference/api-requests/#http-request-headers
    fn setup_headers(&self, builder: reqwest::RequestBuilder, header_params: HeaderParams) -> reqwest::RequestBuilder {
        let mut headers = HeaderMap::new();

        headers.append(header::ACCEPT, "application/json".parse().unwrap());

        if let Some(token) = &self.auth.access_token {
            headers.append(header::AUTHORIZATION, format!("Bearer {}", token.access_token).as_str().parse().unwrap());
        }

        if let Some(merchant_payer_id) = header_params.merchant_payer_id {
            let claims = AuthAssertionClaims {
                iss: self.auth.client_id.clone(),
                payer_id: merchant_payer_id
            };
            let jwt_header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
            let token = jsonwebtoken::encode(&jwt_header, &claims, &jsonwebtoken::EncodingKey::from_secret(self.auth.secret.as_ref())).unwrap();
            let encoded_token = base64::encode(token);
            headers.append("PayPal-Auth-Assertion", encoded_token.as_str().parse().unwrap());
        }

        if let Some(client_metadata_id) = header_params.client_metadata_id {
            headers.append("PayPal-Client-Metadata-Id", client_metadata_id.as_str().parse().unwrap());
        }

        if let Some(partner_attribution_id) = header_params.partner_attribution_id {
            headers.append("PayPal-Partner-Attribution-Id", partner_attribution_id.as_str().parse().unwrap());
        }

        if let Some(request_id) = header_params.request_id {
            headers.append("PayPal-Request-Id", request_id.as_str().parse().unwrap());
        }

        if let Some(prefer) = header_params.prefer {
            match prefer {
                Prefer::Minimal => headers.append("Prefer", "return=minimal".parse().unwrap()),
                Prefer::Representation => headers.append("Prefer", "return=representation".parse().unwrap()),
            };
        }

        if let Some(content_type) = header_params.content_type {
            headers.append(header::CONTENT_TYPE, content_type.as_str().parse().unwrap());
        }

        builder.headers(headers) 
    }

    /// Gets a access token used in all the api calls.
    pub async fn get_access_token(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .client
            .post(format!("{}/v1/oauth2/token", self.endpoint()).as_str())
            .basic_auth(
                self.auth.client_id.as_str(),
                Some(self.auth.secret.as_str()),
            )
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
