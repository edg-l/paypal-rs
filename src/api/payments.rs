//! Call the Payments API to authorize payments, capture authorized payments, refund payments that have already been captured, and show payment information.
//!
//! Reference: <https://developer.paypal.com/docs/api/payments/v2/>

use std::borrow::Cow;

use derive_builder::Builder;

use crate::{data::payment::*, endpoint::Endpoint};

/// Generates the next invoice number that is available to the merchant.
///
/// The next invoice number uses the prefix and suffix from the last invoice number and increments the number by one.
///
/// For example, the next invoice number after `INVOICE-1234` is `INVOICE-1235`.
#[derive(Debug, Default, Clone, Builder)]
pub struct GetAuthorizedPayment {
    /// The ID of the authorized payment for which to show details.
    pub authorization_id: String,
}

impl GetAuthorizedPayment {
    /// New constructor.
    pub fn new(authorization_id: impl ToString) -> Self {
        Self {
            authorization_id: authorization_id.to_string(),
        }
    }
}

impl Endpoint for GetAuthorizedPayment {
    type Query = ();

    type Body = ();

    type Response = AuthorizedPaymentDetails;

    fn relative_path(&self) -> Cow<str> {
        Cow::Owned(format!("/v2/payments/authorizations/{}", self.authorization_id))
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}
