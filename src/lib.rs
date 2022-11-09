//!
//! [![Version](https://img.shields.io/crates/v/paypal-rs)](https://crates.io/crates/paypal-rs)
//! [![Downloads](https://img.shields.io/crates/d/paypal-rs)](https://crates.io/crates/paypal-rs)
//! [![License](https://img.shields.io/crates/l/paypal-rs)](https://crates.io/crates/paypal-rs)
//! ![Rust](https://github.com/edg-l/paypal-rs/workflows/Rust/badge.svg)
//! [![Docs](https://docs.rs/paypal-rs/badge.svg)](https://docs.rs/paypal-rs)
//!
//! A rust library that wraps the [paypal api](https://developer.paypal.com/docs/api) asynchronously in a strongly typed manner.
//!
//! If there is a missing endpoint that you need, you may try to implement the [Endpoint](endpoint::Endpoint) and pass it to [Client::execute](client::Client::execute)
//!
//! Currently in early development.
//!

//! ## Example
//!
//! ```rust
//! use paypal_rs::{
//!     Client,
//!     api::orders::*,
//!     data::orders::*,
//!     data::common::Currency,
//! };
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenvy::dotenv().ok();
//!     let clientid = std::env::var("PAYPAL_CLIENTID").unwrap();
//!     let secret = std::env::var("PAYPAL_SECRET").unwrap();
//!
//!     let mut client = Client::new(clientid, secret, true);
//!
//!     client.get_access_token().await.unwrap();
//!
//!     let order = OrderPayloadBuilder::default()
//!         .intent(Intent::Authorize)
//!         .purchase_units(vec![PurchaseUnit::new(Amount::new(Currency::EUR, "10.0"))])
//!         .build().unwrap();
//!
//!     let create_order = CreateOrder::new(order);
//!    
//!     let _order_created = client
//!         .execute(&create_order).await.unwrap();
//! }
//! ```
//!
//! ## Testing
//! You need the enviroment variables PAYPAL_CLIENTID and PAYPAL_SECRET to be set.
//!
//! `cargo test`
//!
//! ## Roadmap
//!
//! - [x] Orders API - 0.1.0
//! - - [x] Create order
//! - - [x] Update order
//! - - [x] Show order details
//! - - [x] Authorize payment for order
//! - - [x] Capture payment for order
//! - [x] Invoicing API - 0.2.0
//! - - [x] Generate Invoice number
//! - - [x] Create Draft Invoice
//! - - [x] Show Invoice Details (Get Invoice)
//! - - [x] List Invoices
//! - - [x] Delete Invoice
//! - - [x] Update Invoice
//! - - [x] Cancel Invoice
//! - - [x] Send Invoice
//! - - [ ] Send Invoice Reminder
//! - - [ ] List Templates
//! - - [ ] Create Template
//! - - [ ] Delete Template
//! - - [ ] Fully Update Template
//! - - [ ] Show Template Template
//! - [ ] Payments API - 0.3.0
//! - [ ] Tracking API - 0.4.0
//! - [ ] Subscriptions API - 0.5.0
//! - [ ] Identity API - 0.6.0
//! - [ ] Disputes API - 0.7.0
//! - [ ] Catalog Products API - 0.8.0
//! - [ ] Partner Referrals API - 0.9.0
//! - [ ] Payouts API - 0.10.0
//! - [ ] Transaction Search API - 0.11.0
//! - [ ] Referenced Payouts API - 0.12.0
//! - [ ] Vault API - 0.13.0
//! - [ ] Webhooks Management API - 0.14.0
//! - [ ] Payment Experience Web Profiles API - 1.0.0

#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub mod api;
pub mod client;
pub mod countries;
pub mod data;
pub mod endpoint;
pub mod errors;
pub use client::*;

use derive_builder::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;

/// The paypal api endpoint used on a live application.
pub const LIVE_ENDPOINT: &str = "https://api-m.paypal.com";
/// The paypal api endpoint used on when testing.
pub const SANDBOX_ENDPOINT: &str = "https://api-m.sandbox.paypal.com";
/// Represents the query used in most GET api requests.
///
/// Reference: <https://developer.paypal.com/docs/api/reference/api-requests/#query-parameters>
///
/// Note: You can avoid most fields by the Default impl like so:
/// ```
/// use paypal_rs::Query;
/// let query = Query { count: Some(40), ..Default::default() };
/// ```
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Builder, Clone)]
pub struct Query {
    /// The number of items to list in the response.
    pub count: Option<i32>,
    /// The end date and time for the range to show in the response.
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The page number indicating which set of items will be returned in the response.
    /// So, the combination of page=1 and page_size=20 returns the first 20 items.
    /// The combination of page=2 and page_size=20 returns items 21 through 40.
    pub page: Option<i32>,
    /// The number of items to return in the response.
    pub page_size: Option<i32>,
    /// Indicates whether to show the total count in the response.
    pub total_count_required: Option<bool>,
    /// Sorts the payments in the response by a specified value, such as the create time or update time.
    pub sort_by: Option<String>,
    /// Sorts the items in the response in ascending or descending order.
    pub sort_order: Option<String>,
    /// The ID of the starting resource in the response.
    /// When results are paged, you can use the next_id value as the start_id to continue with the next set of results.
    pub start_id: Option<String>,
    /// The start index of the payments to list. Typically, you use the start_index to jump to a specific position in the resource history based on its cart.
    /// For example, to start at the second item in a list of results, specify start_index=2.
    pub start_index: Option<i32>,
    /// The start date and time for the range to show in the response.
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    // TODO: Use https://github.com/samscott89/serde_qs
}

/// Represents the optional header values used on paypal requests.
///
/// <https://developer.paypal.com/docs/api/reference/api-requests/#paypal-auth-assertion>
#[derive(Debug, Default, Builder, Clone)]
pub struct HeaderParams {
    /// The merchant payer id used on PayPal-Auth-Assertion
    pub merchant_payer_id: Option<String>,
    /// Verifies that the payment originates from a valid, user-consented device and application.
    /// Reduces fraud and decreases declines. Transactions that do not include a client metadata ID are not eligible for PayPal Seller Protection.
    pub client_metadata_id: Option<String>,
    /// Identifies the caller as a PayPal partner. To receive revenue attribution, specify a unique build notation (BN) code.
    /// BN codes provide tracking on all transactions that originate or are associated with a particular partner.
    pub partner_attribution_id: Option<String>,
    /// Contains a unique user-generated ID that the server stores for a period of time. Use this header to enforce idempotency on REST API POST calls.
    /// You can make these calls any number of times without concern that the server creates or completes an action on a resource more than once.
    /// You can retry calls that fail with network timeouts or the HTTP 500 status code. You can retry calls for as long as the server stores the ID.
    pub request_id: Option<String>,
    /// The media type. Required for operations with a request body.
    pub content_type: Option<String>,
}

#[derive(Debug, Serialize)]
struct AuthAssertionClaims {
    pub iss: String,
    pub payer_id: String,
}

#[cfg(test)]
mod tests {
    use crate::countries::Country;
    use crate::data::common::Currency;
    use crate::Client;
    use std::env;
    use std::str::FromStr;

    pub async fn create_client() -> Client {
        dotenvy::dotenv().ok();
        let clientid = env::var("PAYPAL_CLIENTID").unwrap();
        let secret = env::var("PAYPAL_SECRET").unwrap();

        Client::new(clientid, secret, crate::PaypalEnv::Sandbox)
    }

    #[test]
    fn test_currency() {
        assert_eq!(Currency::EUR.to_string(), "EUR");
        assert_eq!(Currency::JPY.to_string(), "JPY");
        assert_eq!(Currency::JPY, Currency::from_str("JPY").unwrap());
    }

    #[test]
    fn test_country() {
        assert_eq!(Country::US.to_string(), "US");
        assert_eq!(Country::ES.to_string(), "ES");
        assert_eq!(Country::ES, Country::from_str("ES").unwrap());
    }
}
