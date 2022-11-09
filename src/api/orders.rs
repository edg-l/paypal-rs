//! An order represents a payment between two or more parties. Use the Orders API to create, update, retrieve, authorize, and capture orders.
//!
//! <https://developer.paypal.com/docs/api/orders/v2/>

use std::borrow::Cow;

use derive_builder::Builder;
use serde::Serialize;

use crate::{
    data::orders::{Order, OrderPayload},
    endpoint::Endpoint,
};

/// Creates an order.
#[derive(Debug)]
pub struct CreateOrder {
    /// The order payload.
    pub order: OrderPayload,
}

impl CreateOrder {
    /// New constructor.
    pub fn new(order: OrderPayload) -> Self {
        Self { order }
    }
}

impl Endpoint for CreateOrder {
    type Query = ();

    type Body = OrderPayload;

    type Response = Order;

    fn relative_path(&self) -> Cow<str> {
        Cow::Borrowed("/v2/checkout/orders")
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(self.order.clone())
    }
}

/// Query an order by id.
#[derive(Debug)]
pub struct ShowOrderDetails {
    /// The order id.
    pub order_id: String,
}

impl ShowOrderDetails {
    /// New constructor.
    pub fn new(order_id: &str) -> Self {
        Self {
            order_id: order_id.to_string(),
        }
    }
}

impl Endpoint for ShowOrderDetails {
    type Query = ();

    type Body = ();

    type Response = Order;

    fn relative_path(&self) -> Cow<str> {
        Cow::Owned(format!("/v2/checkout/orders/{}", self.order_id))
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}

/// The payment source used to fund the payment.
#[derive(Debug, Serialize, Builder, Clone)]
pub struct PaymentSourceToken {
    /// The PayPal-generated ID for the token.
    pub id: String,
    /// The tokenization method that generated the ID.
    ///
    /// Can only be BILLING_AGREEMENT.
    pub r#type: String,
}

/// Payment source used in the capture order endpoint.
#[derive(Debug, Serialize, Builder, Clone)]
pub struct PaymentSource {
    /// The tokenized payment source to fund a payment.
    pub token: PaymentSourceToken,
}

/// The capture order endpoint body.
#[derive(Debug, Serialize, Clone, Default)]
pub struct PaymentSourceBody {
    /// The payment source definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_source: Option<PaymentSource>,
}

/// Captures payment for an order. To successfully capture payment for an order,
/// the buyer must first approve the order or a valid payment_source must be provided in the request.
/// A buyer can approve the order upon being redirected to the rel:approve URL that was returned in the HATEOAS links in the create order response.
#[derive(Debug, Clone, Builder)]
pub struct CaptureOrder {
    /// The id of the order.
    pub order_id: String,
    /// The endpoint body.
    pub body: PaymentSourceBody,
}

impl CaptureOrder {
    /// New constructor.
    pub fn new(order_id: &str) -> Self {
        Self {
            order_id: order_id.to_string(),
            body: PaymentSourceBody::default(),
        }
    }
}

impl Endpoint for CaptureOrder {
    type Query = ();

    type Body = PaymentSourceBody;

    type Response = Order;

    fn relative_path(&self) -> Cow<str> {
        Cow::Owned(format!("/v2/checkout/orders/{}/capture", self.order_id))
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(self.body.clone())
    }
}

/// Authorizes payment for an order. To successfully authorize payment for an order,
/// the buyer must first approve the order or a valid payment_source must be provided in the request.
/// A buyer can approve the order upon being redirected to the rel:approve URL that was returned in the HATEOAS links in the create order response.
#[derive(Debug)]
pub struct AuthorizeOrder {
    /// The order id.
    order_id: String,
    /// The endpoint body.
    pub body: PaymentSourceBody,
}

impl AuthorizeOrder {
    /// New constructor.
    pub fn new(order_id: &str) -> Self {
        Self {
            order_id: order_id.to_string(),
            body: PaymentSourceBody::default(),
        }
    }
}

impl Endpoint for AuthorizeOrder {
    type Query = ();

    type Body = PaymentSourceBody;

    type Response = Order;

    fn relative_path(&self) -> Cow<str> {
        Cow::Owned(format!("/v2/checkout/orders/{}/authorize", self.order_id))
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(self.body.clone())
    }
}
/*
#[cfg(test)]
mod tests {
    use crate::data::common::Currency;
    use crate::HeaderParams;
    use crate::{api::orders::*, data::orders::*, tests::create_client};

    #[tokio::test]
    async fn test_order() -> color_eyre::Result<()> {
        let mut client = create_client().await;
        client.get_access_token().await.expect("get access token error");

        let order = OrderPayloadBuilder::default()
            .intent(Intent::Authorize)
            .purchase_units(vec![PurchaseUnit::new(Amount::new(Currency::EUR, "10.0"))])
            .build()?;

        let ref_id = format!(
            "TEST-{:?}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        let create_order = CreateOrder::new(order);

        let order_created = client
            .execute_ext(
                &create_order,
                HeaderParams {
                    request_id: Some(ref_id.clone()),
                    ..Default::default()
                },
            )
            .await;

        assert!(order_created.is_ok());

        let order_created = order_created?;

        assert_ne!(order_created.id, "");
        assert_eq!(order_created.status, OrderStatus::Created);
        assert_eq!(order_created.links.len(), 4);

        let show_order = ShowOrderDetails::new(&order_created.id);

        let show_order_result = client
            .execute_ext(
                &show_order,
                HeaderParams {
                    request_id: Some(ref_id.clone()),
                    ..Default::default()
                },
            )
            .await;

        assert!(show_order_result.is_ok());

        let show_order_result = show_order_result?;

        assert_eq!(order_created.id, show_order_result.id);
        assert_eq!(order_created.status, show_order_result.status);

        let authorize_order = AuthorizeOrder::new(&show_order_result.id);

        let res = client.execute(&authorize_order).await;
        assert!(res.is_err()); // Fails with ORDER_NOT_APPROVED

        Ok(())
    }
}
*/
