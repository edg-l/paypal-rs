//! Paypal object definitions used by the payments api.

use serde::{Deserialize, Serialize};

use super::common::{AuthorizationStatusDetails, LinkDescription, Money, SellerProtection};

/// Payment Status
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
    /// The authorized payment is created. No captured payments have been made for this authorized payment.
    Created,
    /// The authorized payment has one or more captures against it. The sum of these captured payments is greater than the amount of the original authorized payment.
    Captured,
    /// PayPal cannot authorize funds for this authorized payment.
    Denied,
    /// The authorized payment has expired.
    Expired,
    /// A captured payment was made for the authorized payment for an amount that is less than the amount of the original authorized payment.
    PartiallyCaptured,
    /// The payment which was authorized for an amount that is less than the originally requested amount.
    PartiallyCreated,
    /// The authorized payment was voided. No more captured payments can be made against this authorized payment.
    Voided,
    /// The created authorization is in pending state.
    Pending,
}

/// The authorized payment details.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct AuthorizedPaymentDetails {
    /// The status for the authorized payment.
    pub status: PaymentStatus,
    /// The details of the authorized order pending status.
    pub status_details: AuthorizationStatusDetails,
    /// The PayPal-generated ID for the authorized payment.
    pub id: String,
    /// The amount for this authorized payment.
    pub amount: Money,
    /// The API caller-provided external invoice number for this order. Appears in both the payer's transaction history and the emails that the payer receives.
    pub invoice_id: String,
    /// The API caller-provided external ID. Used to reconcile API caller-initiated transactions with PayPal transactions. Appears in transaction and settlement reports.
    pub custom_id: String,
    /// The level of protection offered as defined by PayPal Seller Protection for Merchants.
    pub seller_protection: SellerProtection,
    /// The date and time when the authorized payment expires
    pub expiration_time: chrono::DateTime<chrono::Utc>,
    /// An array of related HATEOAS links.
    pub links: Vec<LinkDescription>,
    /// The date and time when the transaction occurred
    pub create_time: chrono::DateTime<chrono::Utc>,
    /// The date and time when the transaction was last updated
    pub update_time: chrono::DateTime<chrono::Utc>,
}
