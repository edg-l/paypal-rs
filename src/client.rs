//! The paypal api wrapper client, which holds the http request client.

use reqwest::header::{self, HeaderMap};
use serde::Deserialize;
use std::time::Duration;
use std::time::Instant;

use crate::{
    endpoint::Endpoint,
    errors::{PaypalError, ResponseError},
    AuthAssertionClaims, HeaderParams, LIVE_ENDPOINT, SANDBOX_ENDPOINT,
};

/// Represents the access token returned by the OAuth2 authentication.
///
/// <https://developer.paypal.com/docs/api/get-an-access-token-postman/>
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccessToken {
    /// The OAuth2 scopes.
    pub scope: String,
    /// The access token.
    pub access_token: String,
    /// The token type.
    pub token_type: String,
    /// The app id.
    pub app_id: String,
    /// Seconds until it expires.
    pub expires_in: u64,
    /// The nonce.
    pub nonce: String,
}

/// Stores OAuth2 information.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Auth {
    /// Your client id.
    pub client_id: String,
    /// The secret.
    pub secret: String,
    /// The access token returned by oauth2 authentication.
    pub access_token: Option<AccessToken>,
    /// Used to check when the token expires.
    pub expires: Option<(Instant, Duration)>,
}

/// Represents a client used to interact with the paypal api.
#[derive(Debug, Clone)]
pub struct Client {
    /// Internal http client
    pub(crate) client: reqwest::Client,
    /// Whether you are or not in a sandbox enviroment.
    pub env: PaypalEnv,
    /// Api Auth information
    pub auth: Auth,
}

/// The paypal api environment.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PaypalEnv {
    /// The live environment.
    Live,
    /// The sandbox environment.
    Sandbox,
    /// For mocking.
    Mock(String),
}

impl PaypalEnv {
    /// Returns the endpoint of this environment.
    pub fn endpoint(&self) -> &str {
        match &self {
            PaypalEnv::Live => LIVE_ENDPOINT,
            PaypalEnv::Sandbox => SANDBOX_ENDPOINT,
            PaypalEnv::Mock(endpoint) => endpoint.as_str(),
        }
    }

    /// Constructs a url from the target.
    pub fn make_url(&self, target: &str) -> String {
        assert!(target.starts_with('/'), "target path must start with '/'");
        format!("{}{}", self.endpoint(), target)
    }
}

impl Client {
    /// Returns a new client, you must get_access_token afterwards to interact with the api.
    ///
    /// # Examples
    ///
    /// ```
    /// use paypal_rs::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     # dotenv::dotenv().ok();
    ///     let clientid = std::env::var("PAYPAL_CLIENTID").unwrap();
    ///     let secret = std::env::var("PAYPAL_SECRET").unwrap();
    ///
    ///     let mut client = Client::new(
    ///         clientid,
    ///         secret,
    ///         true,
    ///     );
    ///     client.get_access_token().await.unwrap();
    /// }
    /// ```
    pub fn new(client_id: String, secret: String, env: PaypalEnv) -> Client {
        Client {
            client: reqwest::Client::new(),
            env,
            auth: Auth {
                client_id,
                secret,
                access_token: None,
                expires: None,
            },
        }
    }

    /// Sets up the request headers as required on https://developer.paypal.com/docs/api/reference/api-requests/#http-request-headers
    async fn setup_headers(
        &self,
        builder: reqwest::RequestBuilder,
        header_params: HeaderParams,
    ) -> Result<reqwest::RequestBuilder, ResponseError> {
        let mut headers = HeaderMap::new();

        headers.append(header::ACCEPT, "application/json".parse().unwrap());

        if let Some(token) = &self.auth.access_token {
            headers.append(
                header::AUTHORIZATION,
                format!("Bearer {}", token.access_token).parse().unwrap(),
            );
        }

        if let Some(merchant_payer_id) = header_params.merchant_payer_id {
            let claims = AuthAssertionClaims {
                iss: self.auth.client_id.clone(),
                payer_id: merchant_payer_id,
            };
            let jwt_header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
            let token = jsonwebtoken::encode(
                &jwt_header,
                &claims,
                &jsonwebtoken::EncodingKey::from_secret(self.auth.secret.as_ref()),
            )
            .unwrap();
            let encoded_token = base64::encode(token);
            headers.append("PayPal-Auth-Assertion", encoded_token.parse().unwrap());
        }

        if let Some(client_metadata_id) = header_params.client_metadata_id {
            headers.append("PayPal-Client-Metadata-Id", client_metadata_id.parse().unwrap());
        }

        if let Some(partner_attribution_id) = header_params.partner_attribution_id {
            headers.append("PayPal-Partner-Attribution-Id", partner_attribution_id.parse().unwrap());
        }

        if let Some(request_id) = header_params.request_id {
            headers.append("PayPal-Request-Id", request_id.parse().unwrap());
        }

        headers.append("Prefer", "return=representation".parse().unwrap());

        if let Some(content_type) = header_params.content_type {
            headers.append(header::CONTENT_TYPE, content_type.parse().unwrap());
        }

        Ok(builder.headers(headers))
    }

    /// Gets a access token used in all the api calls.
    pub async fn get_access_token(&mut self) -> Result<(), ResponseError> {
        if !self.access_token_expired() {
            return Ok(());
        }
        let res = self
            .client
            .post(self.env.make_url("/v1/oauth2/token"))
            .basic_auth(&self.auth.client_id, Some(&self.auth.secret))
            .header("Content-Type", "x-www-form-urlencoded")
            .header("Accept", "application/json")
            .body("grant_type=client_credentials")
            .send()
            .await
            .map_err(ResponseError::HttpError)?;

        if res.status().is_success() {
            let token = res.json::<AccessToken>().await.map_err(ResponseError::HttpError)?;
            self.auth.expires = Some((Instant::now(), Duration::new(token.expires_in, 0)));
            self.auth.access_token = Some(token);
            Ok(())
        } else {
            Err(ResponseError::ApiError(
                res.json::<PaypalError>().await.map_err(ResponseError::HttpError)?,
            ))
        }
    }

    /// Checks if the access token expired.
    pub fn access_token_expired(&self) -> bool {
        if let Some(expires) = self.auth.expires {
            expires.0.elapsed() >= expires.1
        } else {
            true
        }
    }

    /// Executes the given endpoint with the given headers.
    pub async fn execute_ext<E>(&self, endpoint: &E, headers: HeaderParams) -> Result<E::Response, ResponseError>
    where
        E: Endpoint,
    {
        let mut url = self.env.make_url(&endpoint.relative_path());

        if let Some(query) = endpoint.query() {
            let query_string = serde_qs::to_string(&query).expect("serialize the query correctly");
            url.push_str(&query_string);
        }

        let mut request = self.client.request(endpoint.method(), url);
        request = self.setup_headers(request, headers).await?;

        if let Some(body) = endpoint.body() {
            request = request.json(&body);
        }

        let res = request.send().await?;

        if res.status().is_success() {
            // code to debug responses when parse fails.
            //let resp_text = res.text().await?;
            //dbg!(&resp_text);
            //let mut f = std::fs::File::create("output.txt").unwrap();
            //f.write_all(resp_text.as_bytes()).ok();
            //let response_body: E::Response = serde_json::from_str(&resp_text).unwrap();
            let response_body = res.json::<E::Response>().await?;
            Ok(response_body)
        } else {
            Err(ResponseError::ApiError(res.json::<PaypalError>().await?))
        }
    }

    /// Executes the given endpoints with the default headers.
    ///
    /// You must remember to call [Client::get_access_token] first or this may fail due to not being authed.
    pub async fn execute<E>(&self, endpoint: &E) -> Result<E::Response, ResponseError>
    where
        E: Endpoint,
    {
        self.execute_ext(endpoint, HeaderParams::default()).await
    }
}
