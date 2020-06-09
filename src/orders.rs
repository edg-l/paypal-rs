/// The intent to either capture payment immediately or authorize a payment for an order after order creation. 
#[derive(Debug)]
pub enum Intent {
    /// The merchant intends to capture payment immediately after the customer makes a payment.
    Capture,
    /// The merchant intends to authorize a payment and place funds on hold after the customer makes a payment. 
    /// Authorized payments are guaranteed for up to three days but are available to capture for up to 29 days. 
    /// After the three-day honor period, the original authorized payment expires and you must re-authorize the payment.
    /// You must make a separate request to capture payments on demand. 
    /// This intent is not supported when you have more than one `purchase_unit` within your order.
    Authorize,
}

/// Represents a payer name.
///
/// https://developer.paypal.com/docs/api/orders/v2/#definition-payer.name
#[derive(Debug)]
pub struct PayerName {
    /// When the party is a person, the party's given, or first, name. 
    pub given_name: String,
    /// When the party is a person, the party's surname or family name. Also known as the last name. 
    /// Required when the party is a person. Use also to store multiple surnames including the matronymic, or mother's, surname.
    pub surname: String,
}

/// The phone type.
///
/// https://developer.paypal.com/docs/api/orders/v2/#definition-phone_with_type
#[derive(Debug)]
pub enum PhoneType {
    Fax,
    Home,
    Mobile,
    Other,
    Pager
}

#[derive(Debug)]
pub struct PhoneNumber {
    /// The national number, in its canonical international E.164 numbering plan format. 
    /// The combined length of the country calling code (CC) and the national number must not be greater than 15 digits. 
    /// The national number consists of a national destination code (NDC) and subscriber number (SN). 
    pub national_number: String,
}

/// The phone number of the customer. Available only when you enable the 
/// Contact Telephone Number option in the Profile & Settings for the merchant's PayPal account. 
#[derive(Debug)]
pub struct Phone {
    pub phone_type: Option<PhoneType>,
    pub phone_number: PhoneNumber,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum TaxIdType {
    /// The individual tax ID type.
    BR_CPF,
    /// The business tax ID type.
    BR_CNPJ,
}

#[derive(Debug)]
pub struct TaxInfo {
    /// The customer's tax ID. Supported for the PayPal payment method only.
    /// Typically, the tax ID is 11 characters long for individuals and 14 characters long for businesses. 
    pub tax_id: String,
    /// The customer's tax ID type. Supported for the PayPal payment method only.
    pub tax_id_type: TaxIdType,
}

#[derive(Debug, Default)]
pub struct Address {
    /// The first line of the address. For example, number or street. For example, 173 Drury Lane.
    /// Required for data entry and compliance and risk checks. Must contain the full address. 
    address_line_1: Option<String>,
    /// The second line of the address. For example, suite or apartment number. 
    address_line_2: Option<String>,
    /// A city, town, or village. Smaller than admin_area_level_1. 
    admin_area_2: Option<String>,
    /// The highest level sub-division in a country, which is usually a province, state, or ISO-3166-2 subdivision.
    /// Format for postal delivery. For example, CA and not California.
    admin_area_1: Option<String>,
    /// The postal code, which is the zip code or equivalent. Typically required for countries with a postal code or an equivalent. 
    postal_code: Option<String>,
    /// The two-character [ISO 3166-1](https://developer.paypal.com/docs/api/reference/country-codes/) code that identifies the country or region.
    country_code: String,
}

/// The customer who approves and pays for the order. The customer is also known as the payer. 
///
/// https://developer.paypal.com/docs/api/orders/v2/#definition-payer
#[derive(Debug, Default)]
pub struct Payer {
    /// The name of the payer.
    pub name: Option<PayerName>,
    /// The email address of the payer. 
    pub email_address: Option<String>,
    /// The PayPal-assigned ID for the payer.
    pub payer_id: Option<String>,
    /// The phone number of the customer. Available only when you enable the Contact 
    /// Telephone Number option in the Profile & Settings for the merchant's PayPal account.
    pub phone: Option<Phone>,
    /// The birth date of the payer in YYYY-MM-DD format. 
    pub birth_date: Option<String>,
    /// The tax information of the payer. Required only for Brazilian payer's.
    pub tax_info: Option<TaxInfo>,
    /// The address of the payer.
    pub address: Option<Address>,
}

// TODO: Finish order https://developer.paypal.com/docs/api/orders/v2/