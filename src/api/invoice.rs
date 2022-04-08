//! Use the Invoicing API to create, send, and manage invoices.
//! You can also use the API or webhooks to track invoice payments. When you send an invoice to a customer,
//! the invoice moves from draft to payable state. PayPal then emails the customer a link to the invoice on the PayPal website.
//! Customers with a PayPal account can log in and pay the invoice with PayPal. Alternatively,
//! customers can pay as a guest with a debit card or credit card. For more information, see the Invoicing Overview and the Invoicing Integration Guide.
//!
//! Reference: https://developer.paypal.com/docs/api/invoicing/v2/


/* 

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
            //println!("{:#?}", res.text().await?);
            let inv = res.json::<Invoice>().await?;
            Ok(inv)
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
    pub async fn delete_invoice(&mut self, invoice_id: &str, header_params: HeaderParams) -> Result<(), ResponseError> {
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

#[cfg(test)]
mod tests {
    use crate::{Client, HeaderParams};

    async fn create_client() -> Client {
        dotenv::dotenv().ok();
        let clientid = std::env::var("PAYPAL_CLIENTID").unwrap();
        let secret = std::env::var("PAYPAL_SECRET").unwrap();

        Client::new(clientid, secret, true)
    }

    #[tokio::test]
    async fn test_invoice() -> anyhow::Result<()> {
        let mut client = create_client().await;

        let _list = client.list_invoices(1, 10, HeaderParams::default()).await?;
        Ok(())
    }
}

*/