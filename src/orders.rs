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

impl Default for Intent {
    fn default() -> Self {
        Intent::Capture
    }
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
    Pager,
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

#[derive(Debug)]
pub struct Money {
    /// The [three-character ISO-4217 currency code](https://developer.paypal.com/docs/integration/direct/rest/currency-codes/) that identifies the currency.
    currency_code: String,
    /// The value, which might be:
    /// - An integer for currencies like JPY that are not typically fractional.
    /// - A decimal fraction for currencies like TND that are subdivided into thousandths.
    ///
    /// For the required number of decimal places for a currency code, see [Currency Codes](https://developer.paypal.com/docs/api/reference/currency-codes/).
    value: String,
}

/// Breakdown provides details such as total item amount, total tax amount, shipping, handling, insurance, and discounts, if any.
#[derive(Debug, Default)]
pub struct Breakdown {
    /// The subtotal for all items. Required if the request includes purchase_units[].items[].unit_amount.
    /// Must equal the sum of (items[].unit_amount * items[].quantity) for all items.
    item_total: Option<Money>,
    /// The shipping fee for all items within a given purchase_unit.
    shipping: Option<Money>,
    /// The handling fee for all items within a given purchase_unit.
    handling: Option<Money>,
    /// The total tax for all items. Required if the request includes purchase_units.items.tax. Must equal the sum of (items[].tax * items[].quantity) for all items.
    tax_total: Option<Money>,
    /// The insurance fee for all items within a given purchase_unit.
    insurance: Option<Money>,
    /// The shipping discount for all items within a given purchase_unit.
    shipping_discount: Option<Money>,
    /// The discount for all items within a given purchase_unit.
    discount: Option<Money>,
}

#[derive(Debug, Default)]
pub struct Amount {
    /// The [three-character ISO-4217 currency code](https://developer.paypal.com/docs/integration/direct/rest/currency-codes/) that identifies the currency.
    currency_code: String,
    /// The value, which might be:
    /// - An integer for currencies like JPY that are not typically fractional.
    /// - A decimal fraction for currencies like TND that are subdivided into thousandths.
    ///
    /// For the required number of decimal places for a currency code, see [Currency Codes](https://developer.paypal.com/docs/api/reference/currency-codes/).
    value: String,
    /// The breakdown of the amount.
    breakdown: Option<Breakdown>,
}

#[derive(Debug, Default)]
pub struct Payee {
    /// The email address of merchant.
    email_address: Option<String>,
    /// The encrypted PayPal account ID of the merchant.
    merchant_id: Option<String>,
}

#[derive(Debug)]
pub struct PlatformFee {
    amount: Money,
    payee: Option<Payee>,
}

#[derive(Debug)]
pub enum DisbursementMode {
    /// The funds are released to the merchant immediately.
    Instant,
    /// The funds are held for a finite number of days. The actual duration depends on the region and type of integration. 
    /// You can release the funds through a referenced payout. 
    /// Otherwise, the funds disbursed automatically after the specified duration.
    Delayed,
}

impl Default for DisbursementMode {
    fn default() -> Self {
        DisbursementMode::Instant
    }
}

#[derive(Debug, Default)]
pub struct PaymentInstruction {
    /// An array of various fees, commissions, tips, or donations. 
    platform_fees: Option<Vec<PlatformFee>>,
    /// The funds that are held on behalf of the merchant.
    disbursement_mode: Option<DisbursementMode>
}

#[derive(Debug)]
pub enum ItemCategoryType {
    /// Goods that are stored, delivered, and used in their electronic format.
    /// This value is not currently supported for API callers that leverage 
    /// the [PayPal for Commerce Platform](https://www.paypal.com/us/webapps/mpp/commerce-platform) product.
    Digital,
    /// A tangible item that can be shipped with proof of delivery.
    Physical,
}

impl Default for ItemCategoryType {
    fn default() -> Self {
        ItemCategoryType::Digital
    }
}

#[derive(Debug, Default)]
pub struct ShippingDetail {
    name: Option<String>,
    address: Option<Address>,
}

#[derive(Debug)]
pub struct Item {
    /// The item name or title.
    name: String,
    /// The item price or rate per unit.
    /// If you specify unit_amount, purchase_units[].amount.breakdown.item_total is required. Must equal unit_amount * quantity for all items. 
    unit_amount: Money,
    /// The item tax for each unit. If tax is specified, purchase_units[].amount.breakdown.tax_total is required. Must equal tax * quantity for all items. 
    tax: Option<Money>,
    /// The item quantity. Must be a whole number. 
    quantity: String,
    /// The detailed item description. 
    description: Option<String>,
    /// The stock keeping unit (SKU) for the item. 
    sku: Option<String>,
    /// The item category type
    category: Option<ItemCategoryType>,
}

#[derive(Debug, Default)]
pub struct PurchaseUnitRequest {
    /// The API caller-provided external ID for the purchase unit. Required for multiple purchase units when you must update the order through PATCH.
    /// If you omit this value and the order contains only one purchase unit, PayPal sets this value to default.
    reference_id: Option<String>,
    /// The total order amount with an optional breakdown that provides details, such as the total item amount,
    /// total tax amount, shipping, handling, insurance, and discounts, if any.
    ///
    /// If you specify amount.breakdown, the amount equals item_total plus tax_total plus shipping plus handling plus insurance minus shipping_discount minus discount.
    ///
    /// The amount must be a positive number. For listed of supported currencies and decimal precision,
    /// see the PayPal REST APIs [Currency Codes](https://developer.paypal.com/docs/integration/direct/rest/currency-codes/).
    amount: Amount,
    /// The merchant who receives payment for this transaction.
    payee: Option<Payee>,
    /// Any additional payment instructions for PayPal Commerce Platform customers.
    /// Enables features for the PayPal Commerce Platform, such as delayed disbursement and collection of a platform fee.
    /// Applies during order creation for captured payments or during capture of authorized payments. 
    payment_instruction: Option<PaymentInstruction>,
    /// The purchase description.
    description: Option<String>,
    /// The API caller-provided external ID. Used to reconcile client transactions with PayPal transactions.
    /// Appears in transaction and settlement reports but is not visible to the payer. 
    custom_id: Option<String>,
    /// The API caller-provided external invoice number for this order.
    /// Appears in both the payer's transaction history and the emails that the payer receives. 
    invoice_id: Option<String>,
    /// The soft descriptor is the dynamic text used to construct the statement descriptor that appears on a payer's card statement.
    ///
    /// More info here: https://developer.paypal.com/docs/api/orders/v2/#definition-purchase_unit_request
    soft_descriptor: Option<String>,
    /// An array of items that the customer purchases from the merchant. 
    items: Option<Vec<Item>>,
    /// The name and address of the person to whom to ship the items. 
    shipping: Option<ShippingDetail>,
}

/// The type of landing page to show on the PayPal site for customer checkout. 
#[derive(Debug)]
pub enum LandingPage {
    /// When the customer clicks PayPal Checkout, the customer is redirected to a page to log in to PayPal and approve the payment.
    Login,
    /// When the customer clicks PayPal Checkout, the customer is redirected to a page 
    /// to enter credit or debit card and other relevant billing information required to complete the purchase.
    Billing,
    /// When the customer clicks PayPal Checkout, the customer is redirected to either a page to log in to PayPal and approve 
    /// the payment or to a page to enter credit or debit card and other relevant billing information required to complete the purchase, 
    /// depending on their previous interaction with PayPal.
    NoPreference,
}

impl Default for LandingPage {
    fn default() -> Self {
        LandingPage::NoPreference
    }
}

/// The shipping preference
#[derive(Debug)]
pub enum ShippingPreference {
    /// Use the customer-provided shipping address on the PayPal site.
    GetFromFile,
    /// Redact the shipping address from the PayPal site. Recommended for digital goods.
    NoShipping,
    ///  Use the merchant-provided address. The customer cannot change this address on the PayPal site.
    SetProvidedAddress,
}

impl Default for ShippingPreference {
    fn default() -> Self {
        ShippingPreference::GetFromFile
    }
}

#[derive(Debug)]
pub enum UserAction {
    /// After you redirect the customer to the PayPal payment page, a Continue button appears. Use this option when
    /// the final amount is not known when the checkout flow is initiated and you want to redirect the customer
    /// to the merchant page without processing the payment.
    Continue,
    /// After you redirect the customer to the PayPal payment page, a Pay Now button appears.
    /// Use this option when the final amount is known when the checkout is initiated and you want to 
    /// process the payment immediately when the customer clicks Pay Now.
    PayNow,
}

impl Default for UserAction {
    fn default() -> Self {
        UserAction::Continue
    }
}

#[derive(Debug)]
pub enum PayeePreferred {
    /// Accepts any type of payment from the customer.
    Unrestricted,
    /// Accepts only immediate payment from the customer.
    /// For example, credit card, PayPal balance, or instant ACH. 
    /// Ensures that at the time of capture, the payment does not have the `pending` status.
    ImmediatePaymentRequired,
}

impl Default for PayeePreferred {
    fn default() -> Self {
        PayeePreferred::Unrestricted
    }
}

#[derive(Debug, Default)]
pub struct PaymentMethod {
    payer_selected: Option<String>,
    payer_prefered: Option<PayeePreferred>,
}

#[derive(Debug, Default)]
pub struct ApplicationContext {
    /// The label that overrides the business name in the PayPal account on the PayPal site. 
    brand_name: Option<String>,
    /// The BCP 47-formatted locale of pages that the PayPal payment experience shows. PayPal supports a five-character code.
    ///
    /// For example, da-DK, he-IL, id-ID, ja-JP, no-NO, pt-BR, ru-RU, sv-SE, th-TH, zh-CN, zh-HK, or zh-TW. 
    locale: Option<String>,
    /// The type of landing page to show on the PayPal site for customer checkout
    landing_page: Option<LandingPage>,
    /// The shipping preference
    shipping_preference: Option<ShippingPreference>,
    /// Configures a Continue or Pay Now checkout flow.
    user_action: Option<UserAction>,
    /// The customer and merchant payment preferences. 
    payment_method: Option<PaymentMethod>,
    /// The URL where the customer is redirected after the customer approves the payment. 
    return_url: Option<String>,
    /// The URL where the customer is redirected after the customer cancels the payment. 
    cancel_url: Option<String>,
}

#[derive(Debug, Default)]
pub struct OrderPayload {
    /// The intent to either capture payment immediately or authorize a payment for an order after order creation.
    intent: Intent,
    /// The customer who approves and pays for the order. The customer is also known as the payer. 
    payer: Option<Payer>,
    /// An array of purchase units. Each purchase unit establishes a contract between a payer and the payee.
    /// Each purchase unit represents either a full or partial order that the payer intends to purchase from the payee. 
    purchase_units: Vec<PurchaseUnitRequest>,
    /// Customize the payer experience during the approval process for the payment with PayPal. 
    application_context: Option<ApplicationContext>,
}

// TODO: Finish order https://developer.paypal.com/docs/api/orders/v2/
