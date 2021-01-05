//! Common paypal object definitions used amon 2 or more APIs

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::errors::InvalidCurrencyError;

/// The phone type.
///
/// https://developer.paypal.com/docs/api/orders/v2/#definition-phone_with_type
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
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
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AddressDetails {
    /// The street number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_number: Option<String>,
    /// The street name. Just Drury in Drury Lane.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_name: Option<String>,
    /// The street type. For example, avenue, boulevard, road, or expressway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_type: Option<String>,
    /// The delivery service. Post office box, bag number, or post office name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_service: Option<String>,
    /// A named locations that represents the premise. Usually a building name or number or collection of buildings with a common name or number. For example, Craven House.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_name: Option<String>,
    /// The first-order entity below a named building or location that represents the sub-premise.
    /// Usually a single building within a collection of buildings with a common name. Can be a flat, story, floor, room, or apartment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_building: Option<String>,
}

/// The address of the payer.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Address {
    /// The first line of the address. For example, number or street. For example, 173 Drury Lane.
    /// Required for data entry and compliance and risk checks. Must contain the full address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line_1: Option<String>,
    /// The second line of the address. For example, suite or apartment number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line_2: Option<String>,
    /// A city, town, or village. Smaller than admin_area_level_1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_area_2: Option<String>,
    /// The highest level sub-division in a country, which is usually a province, state, or ISO-3166-2 subdivision.
    /// Format for postal delivery. For example, CA and not California.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_area_1: Option<String>,
    /// The postal code, which is the zip code or equivalent. Typically required for countries with a postal code or an equivalent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The two-character [ISO 3166-1](https://developer.paypal.com/docs/api/reference/country-codes/) code that identifies the country or region.
    pub country_code: String,
    /// The non-portable additional address details that are sometimes needed for compliance, risk, or other scenarios where fine-grain address information might be needed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_details: Option<AddressDetails>,
}

/// Represents money
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
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
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LinkDescription {
    /// The complete target URL.
    pub href: String,
    /// The link relation type, which serves as an ID for a link that unambiguously describes the semantics of the link.
    pub rel: String,
    /// The HTTP method required to make the related call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<LinkMethod>,
}

/// ISO-4217 currency codes.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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
    USD
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
            cur => Err(InvalidCurrencyError(cur.to_owned()))
        }
    }
}
