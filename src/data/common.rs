//! Common paypal object definitions used by 2 or more APIs

use crate::errors::InvalidCurrencyError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::str::FromStr;

/// The phone type.
///
/// <https://developer.paypal.com/docs/api/orders/v2/#definition-phone_with_type>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum PhoneType {
    Fax,
    Home,
    Mobile,
    Other,
    Pager,
}

/// The non-portable additional address details
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AddressDetails {
    /// The street number.
    pub street_number: Option<String>,
    /// The street name. Just Drury in Drury Lane.
    pub street_name: Option<String>,
    /// The street type. For example, avenue, boulevard, road, or expressway.
    pub street_type: Option<String>,
    /// The delivery service. Post office box, bag number, or post office name.
    pub delivery_service: Option<String>,
    /// A named locations that represents the premise. Usually a building name or number or collection of buildings with a common name or number. For example, Craven House.
    pub building_name: Option<String>,
    /// The first-order entity below a named building or location that represents the sub-premise.
    /// Usually a single building within a collection of buildings with a common name. Can be a flat, story, floor, room, or apartment.
    pub sub_building: Option<String>,
}

/// The address of the payer.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Address {
    /// The first line of the address. For example, number or street. For example, 173 Drury Lane.
    /// Required for data entry and compliance and risk checks. Must contain the full address.
    pub address_line_1: Option<String>,
    /// The second line of the address. For example, suite or apartment number.
    pub address_line_2: Option<String>,
    /// A city, town, or village. Smaller than admin_area_level_1.
    pub admin_area_2: Option<String>,
    /// The highest level sub-division in a country, which is usually a province, state, or ISO-3166-2 subdivision.
    /// Format for postal delivery. For example, CA and not California.
    pub admin_area_1: Option<String>,
    /// The postal code, which is the zip code or equivalent. Typically required for countries with a postal code or an equivalent.
    pub postal_code: Option<String>,
    /// The two-character [ISO 3166-1](https://developer.paypal.com/docs/api/reference/country-codes/) code that identifies the country or region.
    pub country_code: String,
    /// The non-portable additional address details that are sometimes needed for compliance, risk, or other scenarios where fine-grain address information might be needed.
    pub address_details: Option<AddressDetails>,
}

/// Represents money
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Builder)]
pub struct Money {
    /// The [three-character ISO-4217 currency code](https://developer.paypal.com/docs/integration/direct/rest/currency-codes/) that identifies the currency.
    pub currency_code: Currency,
    /// The value, which might be:
    /// - An integer for currencies like JPY that are not typically fractional.
    /// - A decimal fraction for currencies like TND that are subdivided into thousandths.
    ///
    /// For the required number of decimal places for a currency code, see [Currency Codes](https://developer.paypal.com/docs/api/reference/currency-codes/).
    pub value: String,
}

macro_rules! impl_money {
    ($name:ident, $type:expr) => {
        #[doc=concat!("Creates a instance of Money with the currency ", stringify!($type))]
        pub fn $name(value: impl ToString) -> Self {
            Self {
                currency_code: $type,
                value: value.to_string(),
            }
        }
    };
}

impl Money {
    impl_money!(eur, Currency::EUR);
    impl_money!(usd, Currency::USD);
    impl_money!(brl, Currency::BRL);
    impl_money!(cny, Currency::CNY);
    impl_money!(czk, Currency::CZK);
    impl_money!(jpy, Currency::JPY);
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum LinkMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Connect,
    Options,
    Patch,
}

/// A HTOAES link
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct LinkDescription {
    /// The complete target URL.
    pub href: String,
    /// The link relation type, which serves as an ID for a link that unambiguously describes the semantics of the link.
    pub rel: Option<String>,
    /// The HTTP method required to make the related call.
    pub method: Option<LinkMethod>,
}

/// ISO-4217 currency codes.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum Currency {
    /// Australian dollar
    AUD,
    /// Brazilian real, supported for in country paypal accounts only.
    BRL,
    /// Canadian dollar
    CAD,
    /// Chinese Renmenbi
    CNY,
    /// Czech koruna
    CZK,
    /// Danish krone
    DKK,
    /// Euro
    EUR,
    /// Hong Kong dollar
    HKD,
    /// Hungarian forint, does not support decimals.
    HUF,
    /// Indian rupee, supported for in country paypal india accounts only.
    INR,
    /// Israeli new shekel
    ILS,
    /// Japanese yen, does not support decimals.
    JPY,
    /// Malaysian ringgit
    MYR,
    /// Mexican peso
    MXN,
    /// New Taiwan dollar, does not support decimals.
    TWD,
    /// New Zealand dollar
    NZD,
    /// Norwegian krone
    NOK,
    /// Philippine peso
    PHP,
    /// Polish złoty
    PLN,
    /// Pound sterling
    GBP,
    /// Russian ruble
    RUB,
    /// Singapore dollar
    SGD,
    /// Swedish krona
    SEK,
    /// Swiss franc
    CHF,
    /// Thai baht
    THB,
    /// United States dollar
    USD,
}

impl Default for Currency {
    fn default() -> Self {
        Self::EUR
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

impl FromStr for Currency {
    type Err = InvalidCurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AUD" => Ok(Self::AUD),
            "BRL" => Ok(Self::BRL),
            "CAD" => Ok(Self::CAD),
            "CNY" => Ok(Self::CNY),
            "CZK" => Ok(Self::CZK),
            "DKK" => Ok(Self::DKK),
            "EUR" => Ok(Self::EUR),
            "HKD" => Ok(Self::HKD),
            "HUF" => Ok(Self::HUF),
            "INR" => Ok(Self::INR),
            "ILS" => Ok(Self::ILS),
            "JPY" => Ok(Self::JPY),
            "MYR" => Ok(Self::MYR),
            "MXN" => Ok(Self::MXN),
            "NOK" => Ok(Self::NOK),
            "PHP" => Ok(Self::PHP),
            "PLN" => Ok(Self::PLN),
            "GBP" => Ok(Self::GBP),
            "RUB" => Ok(Self::RUB),
            "SGD" => Ok(Self::SGD),
            "SEK" => Ok(Self::SGD),
            "CHF" => Ok(Self::CHF),
            "THB" => Ok(Self::THB),
            "USD" => Ok(Self::USD),
            cur => Err(InvalidCurrencyError(cur.to_owned())),
        }
    }
}

/// Details about the status of the authorization.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub struct AuthorizationStatusDetails {
    /// The reason why the authorized status is PENDING.
    pub reason: AuthorizationStatusDetailsReason,
}

/// Authorization status reason.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthorizationStatusDetailsReason {
    /// Authorization is pending manual review.
    PendingReview,
}

/// Indicates whether the transaction is eligible for seller protection.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SellerProtectionStatus {
    /// Your PayPal balance remains intact if the customer claims that they did not receive an item or the account holder claims that they did not authorize the payment.
    Eligible,
    /// Your PayPal balance remains intact if the customer claims that they did not receive an item.
    PartiallyEligible,
    /// This transaction is not eligible for seller protection.
    NotEligible,
}

/// The condition that is covered for the transaction.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DisputeCategory {
    /// The payer paid for an item that they did not receive.
    ItemNotReceived,
    /// The payer did not authorize the payment.
    UnauthorizedTransaction,
}

/// The level of protection offered as defined by PayPal Seller Protection for Merchants.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct SellerProtection {
    /// Indicates whether the transaction is eligible for seller protection.
    pub status: SellerProtectionStatus,
    /// An array of conditions that are covered for the transaction.
    pub dispute_categories: Vec<DisputeCategory>,
}
