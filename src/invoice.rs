//! Use the Invoicing API to create, send, and manage invoices.
//! You can also use the API or webhooks to track invoice payments. When you send an invoice to a customer,
//! the invoice moves from draft to payable state. PayPal then emails the customer a link to the invoice on the PayPal website.
//! Customers with a PayPal account can log in and pay the invoice with PayPal. Alternatively,
//! customers can pay as a guest with a debit card or credit card. For more information, see the Invoicing Overview and the Invoicing Integration Guide.
//!
//! Reference: https://developer.paypal.com/docs/api/invoicing/v2/

use crate::common::*;
use crate::countries::Country;
use crate::HeaderParams;
use crate::errors::{ResponseError, PaypalError};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Paypal File reference
#[derive(Debug, Serialize, Deserialize)]
pub struct FileReference {
    /// The ID of the referenced file.
    pub id: String,
    /// The reference URL for the file.
    pub reference_url: String,
    /// Content type
    pub content_type: String,
    /// The date and time when the file was created
    pub create_time: chrono::DateTime<chrono::Utc>,
    /// The size of the file, in bytes.
    pub size: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// The payment term type.
pub enum PaymentTermType {
    /// The payment for the invoice is due upon receipt of the invoice.
    DueOnReceipt,
    /// The payment for the invoice is due on the date specified in the invoice.
    DueOnDateSpecified,
    /// The payment for the invoice is due in 10 days.
    Net10,
    /// The payment for the invoice is due in 15 days.
    Net15,
    /// The payment for the invoice is due in 30 days.
    Net30,
    /// The payment for the invoice is due in 45 days.
    Net45,
    /// The payment for the invoice is due in 60 days.
    Net60,
    /// The payment for the invoice is due in 90 days.
    Net90,
    /// The invoice has no payment due date.
    NoDueDate,
}

/// The payment due date for the invoice.
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTerm {
    /// The payment term. Payment can be due upon receipt, a specified date, or in a set number of days
    pub term_type: PaymentTermType,
    /// The date when the invoice payment is due,
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
}

/// Flow type
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// The flow variation
pub enum FlowType {
    /// The invoice sent to multiple recipients.
    MultipleRecipientsGroup,
    /// The invoice sent as a batch.
    Batch,
    /// The regular invoice sent to single recipient.
    RegularSingle,
}

/// Metadata about a resource
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Metadata {
    /// The date and time when the resource was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The email address of the account that created the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// The date and time when the resource was last edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The email address of the account that last edited the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time when the resource was canceled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The actor who canceled the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_by: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time when the resource was first sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_sent_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time when the resource was last sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sent_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The email address of the account that last sent the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sent_by: Option<String>,
    /// The flow variation that created this invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_flow: Option<FlowType>,
    /// The URL for the invoice payer view hosted on paypal.com.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_view_url: Option<String>,
    /// The URL for the invoice merchant view hosted on paypal.com
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoicer_view_url: Option<String>,
}

/// The details of the invoice. Includes the invoice number, date, payment terms, and audit metadata.
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceDetail {
    /// The reference data. Includes a post office (PO) number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// The three-character ISO-4217 currency code that identifies the currency.
    pub currency_code: String,
    /// A note to the invoice recipient. Also appears on the invoice notification email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// The general terms of the invoice. Can include return or cancellation policy and other terms and conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_and_conditions: Option<String>,
    /// A private bookkeeping memo for the user.
    pub memo: Option<String>,
    /// An array of PayPal IDs for the files that are attached to an invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<FileReference>>,
    /// The invoice number. Default is the number that is auto-incremented number from the last number.
    pub invoice_number: String,
    /// The invoice date as specificed by the sender
    pub invoice_date: chrono::DateTime<chrono::Utc>,
    /// The payment due date for the invoice.
    pub payment_term: PaymentTerm,
    /// The audit metadata
    pub metadata: Metadata,
}

/// A name to be used as recipient, etc.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Name {
    /// The prefix, or title, to the party's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// When the party is a person, the party's given, or first, name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// When the party is a person, the party's surname or family name.
    /// Also known as the last name. Required when the party is a person.
    /// Use also to store multiple surnames including the matronymic, or mother's, surname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    /// When the party is a person, the party's middle name. Use also to store multiple middle names including the patronymic, or father's, middle name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// The suffix for the party's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// DEPRECATED. The party's alternate name. Can be a business name, nickname,
    /// or any other name that cannot be split into first, last name. Required when the party is a business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_full_name: Option<String>,
    /// When the party is a person, the party's full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
}

/// Phone information
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PhoneDetail {
    /// The country calling code (CC), in its canonical international E.164 numbering plan format.
    pub country_code: Country,
    /// The national number, in its canonical international E.164 numbering plan format.
    pub national_number: String,
    /// The extension number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_number: Option<String>,
    /// The phone type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_type: Option<PhoneType>,
}

/// The invoicer information.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InvoicerInfo {
    /// Required. The business name of the party.
    pub business_name: String,
    /// The first and Last name of the recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    /// The invoicer email address, which must be listed in the user's PayPal profile.
    /// If you omit this value, notifications are sent from and to the primary email address but do not appear on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// An array of invoicer's phone numbers. The invoicer can choose to hide the phone number on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<PhoneDetail>>,
    /// The invoicer's website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// The invoicer's tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// Any additional information. Includes business hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_notes: Option<String>,
    /// The full URL to an external logo image. The logo image must not be larger than 250 pixels wide by 90 pixels high.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
}

/// Billing information
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BillingInfo {
    /// Required. The business name of the party.
    pub business_name: String,
    /// The first and Last name of the recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    /// The address of the recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /// The invoice recipient email address. If you omit this value, the invoice is payable and a notification email is not sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// The invoice recipient's phone numbers. Extension number is not supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<PhoneDetail>>,
    /// Any additional information about the recipient. Maximum length: 40.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    /// The language in which to show the invoice recipient's email message. Used only when the recipient does not have a PayPal account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// Contact information
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInformation {
    /// Required. The business name of the party.
    pub business_name: String,
    /// The first and Last name of the recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    /// The address of the recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

/// Recipient information
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RecipientInfo {
    /// The billing information for the invoice recipient. Includes name, address, email, phone, and language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_info: Option<BillingInfo>,
    /// The recipient's shipping information. Includes the user's contact information, which includes name and address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_info: Option<ContactInformation>,
}

/// Tax information
#[derive(Debug, Serialize, Deserialize)]
pub struct Tax {
    /// The name of the tax applied on the invoice items.
    pub name: String,
    /// The tax rate. Value is from 0 to 100. Supports up to five decimal places.
    pub percent: String,
    /// The calculated tax amount. The tax amount is added to the item total.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
}

/// Discount information
#[derive(Debug, Serialize, Deserialize)]
pub struct Discount {
    /// The discount as a percentage value. Value is from 0 to 100. Supports up to five decimal places.
    pub percent: Option<String>,
    /// The invoice level discount amount. Value is from 0 to 1000000. Supports up to two decimal places.
    pub amount: Option<String>,
}

/// The unit of measure for the invoiced item.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnitOffMeasure {
    /// The unit of measure is quantity. This invoice template is typically used for physical goods.
    Quantity,
    /// The unit of measure is hours. This invoice template is typically used for services.
    Hours,
    /// The unit of measure is amount. This invoice template is typically used when only amount is required.
    Amount,
}

/// Item information
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    /// The ID of the invoice line item.
    /// Read only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The item name for the invoice line item.
    pub name: String,
    /// The item description for the invoice line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The quantity of the item that the invoicer provides to the payer. Value is from -1000000 to 1000000. Supports up to five decimal places.
    pub quantity: String,
    /// The unit price of the item. This does not include tax and discount. Value is from -1000000 to 1000000. Supports up to two decimal places.
    pub unit_amount: Money,
    /// The tax associated with the item. The tax amount is added to the item total. Value is from 0 to 100. Supports up to five decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<Tax>,
    /// The date when the item or service was provided, in Internet date and time format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_date: Option<chrono::DateTime<chrono::Utc>>,
    /// Discount as a percent or amount at invoice level. The invoice discount amount is subtracted from the item total.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Discount>,
    /// The unit of measure for the invoiced item. For AMOUNT the unit_amount and quantity are not shown on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<UnitOffMeasure>,
}

/// The partial payment details.
#[derive(Debug, Serialize, Deserialize)]
pub struct PartialPayment {
    /// Indicates whether the invoice allows a partial payment. If false, the invoice must be paid in full. If true, the invoice allows partial payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_partial_payment: Option<bool>,
    /// The minimum amount allowed for a partial payment. Valid only when allow_partial_payment is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount_due: Option<Money>,
}

/// The invoice configuration details. Includes partial payment, tip, and tax calculated after discount.
#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    /// Indicates whether the tax is calculated before or after a discount. If false, the tax is calculated before a discount. If true, the tax is calculated after a discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_calculated_after_discount: Option<bool>,
    /// Indicates whether the unit price includes tax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_inclusive: Option<bool>,
    /// Indicates whether the invoice enables the customer to enter a tip amount during payment.
    /// If true, the invoice shows a tip amount field so that the customer can enter a tip amount. If false, the invoice does not show a tip amount field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_tip: Option<bool>,
    /// The partial payment details. Includes the minimum amount that the invoicer wants the payer to pay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_payment: Option<PartialPayment>,
    /// The template ID. The template determines the layout of the invoice. Includes which fields to show and hide. Default: PayPal system template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

/// The discount
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AggregatedDiscount {
    /// The discount as a percent or amount at invoice level. The invoice discount amount is subtracted from the item total.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_discount: Option<Discount>,
    /// The discount as a percent or amount at item level. The item discount amount is subtracted from each item amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_discount: Option<Money>,
}

/// The shipping fee
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ShippingCost {
    /// The shipping amount. Value is from 0 to 1000000. Supports up to two decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    /// The tax associated with the shipping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<Tax>,
}

/// The custom amount to apply to an invoice
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CustomAmount {
    /// The label to the custom amount of the invoice.
    pub label: String,
    /// The custom amount value. Value is from -1000000 to 1000000. Supports up to two decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
}

/// The breakdown of the amount. Breakdown provides details such as total item amount, total tax amount, custom amount, shipping and discounts, if any.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Breakdown {
    /// The subtotal for all items. Must equal the sum of (items[].unit_amount * items[].quantity) for all items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_total: Option<Money>,
    /// The discount can be at the item or invoice level, or both. Can be applied as a percent or amount. If you provide both amount and percent, amount takes precedent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<AggregatedDiscount>,
    /// The aggregated amount of the item and shipping taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<Money>,
    /// The shipping fee for all items. Includes tax on shipping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingCost>,
    /// The custom amount to apply to an invoice. If you include a label, you must include the custom amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<CustomAmount>,
}

/// Represents an amount of money.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Amount {
    /// The [three-character ISO-4217 currency code](https://developer.paypal.com/docs/integration/direct/rest/currency-codes/) that identifies the currency.
    pub currency_code: String,
    /// The value, which might be:
    /// - An integer for currencies like JPY that are not typically fractional.
    /// - A decimal fraction for currencies like TND that are subdivided into thousandths.
    ///
    /// For the required number of decimal places for a currency code, see [Currency Codes](https://developer.paypal.com/docs/api/reference/currency-codes/).
    pub value: String,
    /// The breakdown of the amount. Breakdown provides details such as total item amount, total tax amount, custom amount, shipping and discounts, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<Breakdown>,
}

impl Amount {
    /// Creates a new amount with the required values.
    pub fn new<S: Into<String>>(currency_code: S, value: S) -> Self {
        Amount {
            currency_code: currency_code.into(),
            value: value.into(),
            breakdown: None,
        }
    }
}

/// The payment type in an invoicing flow
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentType {
    /// The payment type is PayPal.
    Paypal,
    /// The payment type is an external cash or a check payment.
    External,
}

/// The payment mode or method through which the invoicer can accept the payment.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentMethod {
    /// Payments can be received through bank transfers.
    BankTransfer,
    /// Payments can be received as cash.
    Cash,
    /// Payments can be received as check.
    Check,
    /// Payments can be received through credit card payments.
    CreditCard,
    /// Payments can be received through debit card payments.
    DebitCard,
    /// Payments can be received through paypal payments.
    Paypal,
    /// Payments can be received through wire transfer.
    WireTransfer,
    /// Payments can be received through other modes.
    Other,
}

impl Default for PaymentMethod {
    fn default() -> Self {
        PaymentMethod::Paypal
    }
}

/// Payment detail
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDetail {
    /// The payment type in an invoicing flow which can be PayPal or an external cash or check payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PaymentType>,
    /// The ID for a PayPal payment transaction. Required for the PAYPAL payment type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    /// The date when the invoice was paid, in Internet date and time format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<chrono::DateTime<chrono::Utc>>,
    /// The payment mode or method through which the invoicer can accept the payment.
    pub method: PaymentMethod,
    /// A note associated with an external cash or check payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// The payment amount to record against the invoice. If you omit this parameter, the total invoice amount is marked as paid. This amount cannot exceed the amount due.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    /// The recipient's shipping information. Includes the user's contact information, which includes name and address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_info: Option<ContactInformation>,
}

/// Payments registered against the invoice
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Payments {
    /// The aggregated payment amounts against this invoice.
    /// Read only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_amount: Option<Money>,
    /// An array of payment details for the invoice. The payment details of the invoice like payment type, method, date, discount and transaction type.
    /// Read only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<PaymentDetail>>,
}

/// Refund details
#[derive(Debug, Serialize, Deserialize)]
pub struct RefundDetail {
    /// The PayPal refund type. Indicates whether the refund was paid through PayPal or externally in the invoicing flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PaymentType>,
    /// The ID for a PayPal payment transaction. Required for the PAYPAL payment type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_id: Option<String>,
    /// The date when the invoice was refunded, in Internet date format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_date: Option<chrono::DateTime<chrono::Utc>>,
    /// The amount to record as refunded. If you omit the amount, the total invoice paid amount is recorded as refunded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    /// The payment mode or method through which the invoicer can accept the payments.
    pub method: PaymentMethod,
}

/// List of refunds
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Refunds {
    /// The aggregated refund amounts.
    /// Read only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<Money>,
    /// An array of refund details for the invoice. Includes the refund type, date, amount, and method.
    /// Read only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<RefundDetail>>,
}

/// The status of the invoice
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    ///  The invoice is in draft state. It is not yet sent to the payer.
    Draft,
    /// The invoice has been sent to the payer. The payment is awaited from the payer.
    Sent,
    /// The invoice is scheduled on a future date. It is not yet sent to the payer.
    Scheduled,
    /// The payer has paid for the invoice.
    Paid,
    /// The invoice is marked as paid by the invoicer.
    MarkedAsPaid,
    /// The invoice has been cancelled by the invoicer.
    Cancelled,
    /// The invoice has been refunded by the invoicer.
    Refunded,
    /// The payer has partially paid for the invoice.
    PartiallyPaid,
    /// The invoice has been partially refunded by the invoicer.
    PartiallyRefunded,
    /// The invoice is marked as refunded by the invoicer.
    MarkedAsRefunded,
    /// The invoicer is yet to receive the payment from the payer for the invoice.
    Unpaid,
    /// The invoicer is yet to receive the payment for the invoice. It is under pending review.
    PaymentPending,
}

/// An invoice payload
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicePayload {
    /// The details of the invoice. Includes the invoice number, date, payment terms, and audit metadata.
    pub detail: InvoiceDetail,
    /// The invoicer information. Includes the business name, email, address, phone, fax, tax ID, additional notes, and logo URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoicer: Option<InvoicerInfo>,
    /// The billing and shipping information. Includes name, email, address, phone and language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_recipient: Option<Vec<RecipientInfo>>,
    /// An array of one or more CC: emails to which notifications are sent.
    /// If you omit this parameter, a notification is sent to all CC: email addresses that are part of the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_recipients: Option<Vec<String>>,
    /// An array of invoice line item information.
    pub items: Vec<Item>,
    /// The invoice configuration details. Includes partial payment, tip, and tax calculated after discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Configuration>,
    /// The invoice amount summary of item total, discount, tax total and shipping..
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    /// List of payments registered against the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<Payments>,
    /// List of refunds against this invoice. The invoicing refund details includes refund type, date, amount, and method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunds: Option<Refunds>,
}

/// Definition: https://developer.paypal.com/docs/api/invoicing/v2/#invoices_get
#[derive(Debug, Serialize, Deserialize)]
pub struct Invoice {
    /// The ID of the invoice.
    pub id: String,
    /// The parent ID to an invoice that defines the group invoice to which the invoice is related.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// The status of the invoice.
    pub status: Status,
    /// The details of the invoice. Includes the invoice number, date, payment terms, and audit metadata.
    pub detail: InvoiceDetail,
    /// The invoicer information. Includes the business name, email, address, phone, fax, tax ID, additional notes, and logo URL.
    pub invoicer: InvoicerInfo,
    /// The billing and shipping information. Includes name, email, address, phone and language.
    pub primary_recipients: Vec<RecipientInfo>,
    /// An array of one or more CC: emails to which notifications are sent.
    /// If you omit this parameter, a notification is sent to all CC: email addresses that are part of the invoice.
    pub additional_recipients: Option<Vec<String>>,
    /// An array of invoice line item information.
    pub items: Option<Vec<Item>>,
    /// The invoice configuration details. Includes partial payment, tip, and tax calculated after discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Configuration>,
    /// The invoice amount summary of item total, discount, tax total and shipping..
    pub amount: Amount,
    /// The due amount, which is the balance amount outstanding after payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_amount: Option<Money>,
    /// The amount paid by the payer as gratuity to the invoicer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gratuity: Option<Money>,
    /// List of payments registered against the invoice..
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<Payments>,
    /// List of refunds against this invoice. The invoicing refund details includes refund type, date, amount, and method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunds: Option<Refunds>,
    /// An array of request-related HATEOAS links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<LinkDescription>>,
}

/// A invoice list
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceList {
    /// Total items
    pub total_items: i32,
    /// Total pages
    pub total_pages: i32,
    /// The invoices
    pub items: Vec<Invoice>,
    /// HATEOAS links
    pub links: Vec<LinkDescription>,
}

/// Cancel invoice reason
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CancelReason {
    /// The subject of the email that is sent as a notification to the recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// A note to the payer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Indicates whether to send a copy of the email to the merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_to_invoicer: Option<bool>,
    /// Indicates whether to send a copy of the email to the recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_to_recipient: Option<bool>,
    /// An array of one or more CC: emails to which notifications are sent.
    /// If you omit this parameter, a notification is sent to all CC: email addresses that are part of the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_recipients: Option<Vec<String>>,
}

/// QR pay action
pub const QR_ACTION_PAY: &str = "pay";
/// QR details action
pub const QR_ACTION_DETAILS: &str = "details";

/// QR creation parameters
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QRCodeParams {
    /// The width, in pixels, of the QR code image. Value is from 150 to 500.
    pub width: i32,
    /// The height, in pixels, of the QR code image. Value is from 150 to 500.
    pub height: i32,
    /// The type of URL for which to generate a QR code. Valid values are pay and details.
    ///
    /// Check QR_ACTION_PAY and QR_ACTION_DETAILS constants
    pub action: Option<String>,
}

/// Used to record a payment.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RecordPaymentPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_date: Option<chrono::DateTime<chrono::Utc>>,
    method: PaymentMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    amount: Amount,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_info: Option<ContactInformation>,
}

impl super::Client {
    /// Generates the next invoice number that is available to the merchant.
    ///
    /// The next invoice number uses the prefix and suffix from the last invoice number and increments the number by one.
    ///
    /// For example, the next invoice number after `INVOICE-1234` is `INVOICE-1235`.
    pub async fn generate_invoice_number(
        &mut self,
        header_params: crate::HeaderParams,
    ) -> Result<String, ResponseError> {
        let build = self
            .setup_headers(
                self.client
                    .post(format!("{}/v2/invoicing/generate-next-invoice-number", self.endpoint()).as_str()),
                header_params,
            )
            .await;

        let res = build.send().await?;

        if res.status().is_success() {
            let x = res.json::<HashMap<String, String>>().await?;
            Ok(x.get("invoice_number").expect("to have a invoice number").clone())
        } else {
            Err(res.json::<PaypalError>().await?.into())
        }
    }

    /// Creates a draft invoice. To move the invoice from a draft to payable state, you must send the invoice.
    /// Include invoice details including merchant information. The invoice object must include an items array.
    pub async fn create_draft_invoice(
        &mut self,
        invoice: InvoicePayload,
        header_params: HeaderParams,
    ) -> Result<Invoice, ResponseError> {
        let build = self
            .setup_headers(
                self.client
                    .post(format!("{}/v2/invoicing/invoices", self.endpoint()).as_str()),
                header_params,
            )
            .await;

        let res = build.json(&invoice).send().await?;

        if res.status().is_success() {
            let x = res.json::<Invoice>().await?;
            Ok(x)
        } else {
            Err(res.json::<PaypalError>().await?.into())
        }
    }

    /// Get an invoice by ID.
    pub async fn get_invoice(
        &mut self,
        invoice_id: &str,
        header_params: HeaderParams,
    ) -> Result<Invoice, ResponseError> {
        let build = self
            .setup_headers(
                self.client
                    .post(format!("{}/v2/invoicing/invoices/{}", self.endpoint(), invoice_id).as_str()),
                header_params,
            )
            .await;

        let res = build.send().await?;

        if res.status().is_success() {
            let x = res.json::<Invoice>().await?;
            Ok(x)
        } else {
            Err(res.json::<PaypalError>().await?.into())
        }
    }

    /// List invoices
    /// Page size has the following limits: [1, 100].
    pub async fn list_invoices(
        &mut self,
        page: i32,
        page_size: i32,
        header_params: HeaderParams,
    ) -> Result<InvoiceList, ResponseError> {
        let build = self
            .setup_headers(
                self.client.get(
                    format!(
                        "{}/v2/invoicing/invoices?page={}&page_size={}&total_required=true",
                        self.endpoint(),
                        page,
                        page_size
                    )
                    .as_str(),
                ),
                header_params,
            )
            .await;

        let res = build.send().await?;

        if res.status().is_success() {
            let x = res.json::<InvoiceList>().await?;
            Ok(x)
        } else {
            Err(res.json::<PaypalError>().await?.into())
        }
    }

    /// Delete a invoice
    pub async fn delete_invoice(
        &mut self,
        invoice_id: &str,
        header_params: HeaderParams,
    ) -> Result<(), ResponseError> {
        let build = self
            .setup_headers(
                self.client
                    .delete(format!("{}/v2/invoicing/invoices/{}", self.endpoint(), invoice_id).as_str()),
                header_params,
            )
            .await;

        let res = build.send().await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(res.json::<PaypalError>().await?.into())
        }
    }

    /// Update a invoice
    pub async fn update_invoice(
        &mut self,
        invoice: Invoice,
        send_to_recipient: bool,
        send_to_invoicer: bool,
        header_params: HeaderParams,
    ) -> Result<(), ResponseError> {
        let build = self
            .setup_headers(
                self.client.put(
                    format!(
                        "{}/v2/invoicing/invoices/{}?send_to_recipient={}&send_to_invoicer={}",
                        self.endpoint(),
                        invoice.id,
                        send_to_recipient,
                        send_to_invoicer
                    )
                    .as_str(),
                ),
                header_params,
            )
            .await;

        let res = build.send().await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(res.json::<PaypalError>().await?.into())
        }
    }

    /// Cancel a invoice
    pub async fn cancel_invoice(
        &mut self,
        invoice_id: &str,
        reason: CancelReason,
        header_params: HeaderParams,
    ) -> Result<(), ResponseError> {
        let build = self
            .setup_headers(
                self.client
                    .post(format!("{}/v2/invoicing/invoices/{}/cancel", self.endpoint(), invoice_id,).as_str()),
                header_params,
            )
            .await;

        let res = build.json(&reason).send().await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(res.json::<PaypalError>().await?.into())
        }
    }

    /// Generate a QR code
    pub async fn generate_qr_code(
        &mut self,
        invoice_id: &str,
        params: QRCodeParams,
        header_params: HeaderParams,
    ) -> Result<Bytes, ResponseError> {
        let build = self
            .setup_headers(
                self.client.post(
                    format!(
                        "{}/v2/invoicing/invoices/{}/generate-qr-code",
                        self.endpoint(),
                        invoice_id
                    )
                    .as_str(),
                ),
                header_params,
            )
            .await;

        let res = build.json(&params).send().await?;

        if res.status().is_success() {
            let b = res.bytes().await?;
            Ok(b)
        } else {
            Err(res.json::<PaypalError>().await?.into())
        }
    }

    /// Records a payment for the invoice. If no payment is due, the invoice is marked as PAID. Otherwise, the invoice is marked as PARTIALLY PAID.
    pub async fn record_invoice_payment(
        &mut self,
        invoice_id: &str,
        payload: RecordPaymentPayload,
        header_params: crate::HeaderParams,
    ) -> Result<String, ResponseError> {
        let build = self
            .setup_headers(
                self.client
                    .post(format!("{}/v2/invoicing/invoices/{}/payments", self.endpoint(), invoice_id).as_str()),
                header_params,
            )
            .await;

        let res = build.json(&payload).send().await?;

        if res.status().is_success() {
            let x = res.json::<HashMap<String, String>>().await?;
            Ok(x.get("payment_id").unwrap().to_owned())
        } else {
            Err(res.json::<PaypalError>().await?.into())
        }
    }

    // TODO: https://developer.paypal.com/docs/api/invoicing/v2/#invoices_payments-delete
}
