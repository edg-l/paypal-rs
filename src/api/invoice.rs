//! Use the Invoicing API to create, send, and manage invoices.
//! You can also use the API or webhooks to track invoice payments. When you send an invoice to a customer,
//! the invoice moves from draft to payable state. PayPal then emails the customer a link to the invoice on the PayPal website.
//! Customers with a PayPal account can log in and pay the invoice with PayPal. Alternatively,
//! customers can pay as a guest with a debit card or credit card. For more information, see the Invoicing Overview and the Invoicing Integration Guide.
//!
//! Reference: <https://developer.paypal.com/docs/api/invoicing/v2/>

use std::borrow::Cow;

use derive_builder::Builder;
use serde::Serialize;

use crate::{
    data::{
        invoice::{CancelReason, Invoice, InvoiceList, InvoicePayload, SendInvoicePayload},
        orders::InvoiceNumber,
    },
    endpoint::Endpoint,
    Query,
};

/// Generates the next invoice number that is available to the merchant.
///
/// The next invoice number uses the prefix and suffix from the last invoice number and increments the number by one.
///
/// For example, the next invoice number after `INVOICE-1234` is `INVOICE-1235`.
#[derive(Debug, Default, Clone)]
pub struct GenerateInvoiceNumber {
    /// The invoice number. If you omit this value, the default is the auto-incremented number from the last number.
    pub invoice_number: Option<InvoiceNumber>,
}

impl GenerateInvoiceNumber {
    /// New constructor.
    pub fn new(invoice_number: Option<InvoiceNumber>) -> Self {
        Self { invoice_number }
    }
}

impl Endpoint for GenerateInvoiceNumber {
    type Query = ();

    type Body = Option<InvoiceNumber>;

    type Response = InvoiceNumber;

    fn relative_path(&self) -> Cow<str> {
        Cow::Borrowed("/v2/invoicing/generate-next-invoice-number")
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(self.invoice_number.clone())
    }
}

/// Creates a draft invoice. To move the invoice from a draft to payable state, you must send the invoice.
/// Include invoice details including merchant information. The invoice object must include an items array.
#[derive(Debug, Clone)]
pub struct CreateDraftInvoice {
    /// The invoice details.
    pub invoice: InvoicePayload,
}

impl CreateDraftInvoice {
    /// New constructor.
    pub fn new(invoice: InvoicePayload) -> Self {
        Self { invoice }
    }
}

impl Endpoint for CreateDraftInvoice {
    type Query = ();

    type Body = InvoicePayload;

    type Response = Invoice;

    fn relative_path(&self) -> Cow<str> {
        Cow::Borrowed("/v2/invoicing/invoices")
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(self.invoice.clone())
    }
}

/// Shows details for an invoice, by ID.
#[derive(Debug, Clone)]
pub struct GetInvoice {
    /// The invoice id.
    pub invoice_id: String,
}

impl GetInvoice {
    /// New constructor.
    pub fn new(invoice_id: impl ToString) -> Self {
        Self {
            invoice_id: invoice_id.to_string(),
        }
    }
}

impl Endpoint for GetInvoice {
    type Query = ();

    type Body = ();

    type Response = Invoice;

    fn relative_path(&self) -> Cow<str> {
        Cow::Owned(format!("/v2/invoicing/invoices/{}", self.invoice_id))
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}

/// Lists invoices. To filter the invoices that appear in the response, you can specify one or more optional query parameters.
/// Page size has the following limits: [1, 100].
#[derive(Debug, Clone)]
pub struct ListInvoices {
    /// The endpoint query.
    pub query: Query,
}

impl ListInvoices {
    /// New constructor.
    pub fn new(query: Query) -> Self {
        Self { query }
    }
}

impl Endpoint for ListInvoices {
    type Query = Query;

    type Body = ();

    type Response = InvoiceList;

    fn relative_path(&self) -> Cow<str> {
        Cow::Borrowed("/v2/invoicing/invoices")
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(self.query.clone())
    }
}

/// Deletes a draft or scheduled invoice, by ID. Deletes invoices in the draft or scheduled state only.
///
/// For invoices that have already been sent, you can cancel the invoice.
///
/// After you delete a draft or scheduled invoice, you can no longer use it or show its details. However, you can reuse its invoice number.
#[derive(Debug, Clone)]
pub struct DeleteInvoice {
    /// The invocie id.
    pub invoice_id: String,
}

impl DeleteInvoice {
    /// New constructor.
    pub fn new(invoice_id: impl ToString) -> Self {
        Self {
            invoice_id: invoice_id.to_string(),
        }
    }
}

impl Endpoint for DeleteInvoice {
    type Query = Query;

    type Body = ();

    type Response = ();

    fn relative_path(&self) -> Cow<str> {
        Cow::Owned(format!("/v2/invoicing/invoices/{}", self.invoice_id))
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::DELETE
    }
}

/// The update invoice query.
#[derive(Debug, Clone, Serialize, Builder)]
pub struct UpdateInvoiceQuery {
    /// Indicates whether to send the invoice update notification to the recipient.
    pub send_to_recipient: bool,
    /// Indicates whether to send the invoice update notification to the merchant.
    pub send_to_invoicer: bool,
}

/// Update an invoice.
///
/// Fully updates an invoice, by ID. In the JSON request body, include a complete invoice object. This call does not support partial updates.
#[derive(Debug, Clone)]
pub struct UpdateInvoice {
    /// The updated invoice object.
    pub invoice: Invoice,
    /// The update invoice query.
    pub query: UpdateInvoiceQuery,
}

impl UpdateInvoice {
    /// New constructor.
    pub fn new(invoice: Invoice, query: UpdateInvoiceQuery) -> Self {
        Self { invoice, query }
    }
}

impl Endpoint for UpdateInvoice {
    type Query = UpdateInvoiceQuery;

    type Body = Invoice;

    type Response = Invoice;

    fn relative_path(&self) -> Cow<str> {
        Cow::Owned(format!("/v2/invoicing/invoices/{}", self.invoice.id))
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::PUT
    }

    fn body(&self) -> Option<Self::Body> {
        Some(self.invoice.clone())
    }

    fn query(&self) -> Option<Self::Query> {
        Some(self.query.clone())
    }
}

/// Cancels a sent invoice, by ID, and, optionally, sends a notification about the cancellation to the payer, merchant, and CC: emails.
#[derive(Debug, Clone)]
pub struct CancelInvoice {
    /// The invoice id.
    pub invoice_id: String,
    /// The reason of the cancelation.
    pub reason: CancelReason,
}

impl CancelInvoice {
    /// New constructor.
    pub fn new(invoice_id: impl ToString, reason: CancelReason) -> Self {
        Self {
            invoice_id: invoice_id.to_string(),
            reason,
        }
    }
}

impl Endpoint for CancelInvoice {
    type Query = ();

    type Body = CancelReason;

    type Response = ();

    fn relative_path(&self) -> Cow<str> {
        Cow::Owned(format!("/v2/invoicing/invoices/{}/cancel", self.invoice_id))
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(self.reason.clone())
    }
}

/// Sends or schedules an invoice, by ID, to be sent to a customer.
#[derive(Debug, Clone)]
pub struct SendInvoice {
    /// The invoice id.
    pub invoice_id: String,
    /// The payload.
    pub payload: SendInvoicePayload,
}

impl SendInvoice {
    /// New constructor.
    pub fn new(invoice_id: impl ToString, payload: SendInvoicePayload) -> Self {
        Self {
            invoice_id: invoice_id.to_string(),
            payload,
        }
    }
}

impl Endpoint for SendInvoice {
    type Query = ();

    type Body = SendInvoicePayload;

    type Response = ();

    fn relative_path(&self) -> Cow<str> {
        Cow::Owned(format!("/v2/invoicing/invoices/{}/send", self.invoice_id))
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(self.payload.clone())
    }
}

/*

impl super::Client {

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
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::common::*;
    use crate::data::invoice::*;
    use crate::Client;

    async fn create_client() -> Client {
        dotenv::dotenv().ok();
        let clientid = std::env::var("PAYPAL_CLIENTID").unwrap();
        let secret = std::env::var("PAYPAL_SECRET").unwrap();

        let mut client = Client::new(clientid, secret, crate::PaypalEnv::Sandbox);
        client.get_access_token().await.unwrap();
        client
    }

    #[tokio::test]
    async fn test_invoice_create_cancel() -> anyhow::Result<()> {
        let client = create_client().await;

        let payload = InvoicePayloadBuilder::default()
            .detail(InvoiceDetailBuilder::default().currency_code(Currency::EUR).build()?)
            .invoicer(
                InvoicerInfoBuilder::default()
                    .name(NameBuilder::default().full_name("Test Person").build()?)
                    .build()?,
            )
            .items(vec![ItemBuilder::default()
                .name("Some name")
                .unit_amount(Money {
                    currency_code: Currency::EUR,
                    value: "10.0".to_string(),
                })
                .quantity("1")
                .build()?])
            .build()?;

        let invoice = CreateDraftInvoice::new(payload);

        let _res = client.execute(&invoice).await?;
        Ok(())
    }
}
