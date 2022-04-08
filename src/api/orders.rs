use std::borrow::Cow;

use crate::{
    data::orders::{Order, OrderPayload, PaymentSourceResponse},
    endpoint::Endpoint,
};

#[derive(Debug)]
pub struct CreateOrder {
    order: OrderPayload,
}

impl CreateOrder {
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

    fn body(&self) -> Option<&Self::Body> {
        Some(&self.order)
    }
}

// TODO: Update order.

#[derive(Debug)]
pub struct ShowOrderDetails {
    order_id: String,
}

impl ShowOrderDetails {
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

#[derive(Debug)]
pub struct CaptureOrder {
    order_id: String,
    // TODO: payment source? https://developer.paypal.com/docs/api/orders/v2/#orders_capture
}

impl CaptureOrder {
    pub fn new(order_id: &str) -> Self {
        Self {
            order_id: order_id.to_string(),
        }
    }
}

impl Endpoint for CaptureOrder {
    type Query = ();

    type Body = ();

    type Response = Order;

    fn relative_path(&self) -> Cow<str> {
        Cow::Owned(format!("/v2/checkout/orders/{}/capture", self.order_id))
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }
}

#[derive(Debug)]
pub struct AuthorizeOrder {
    order_id: String,
    // TODO: payment source? https://developer.paypal.com/docs/api/orders/v2/#orders_authorize
}

impl AuthorizeOrder {
    pub fn new(order_id: &str) -> Self {
        Self {
            order_id: order_id.to_string(),
        }
    }
}

impl Endpoint for AuthorizeOrder {
    type Query = ();

    type Body = ();

    type Response = Order;

    fn relative_path(&self) -> Cow<str> {
        Cow::Owned(format!("/v2/checkout/orders/{}/authorize", self.order_id))
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }
}

#[cfg(test)]
mod tests {
    use crate::data::common::Currency;
    use crate::HeaderParams;
    use crate::{
        api::orders::{CreateOrder, ShowOrderDetails},
        data::orders::*,
        tests::create_client,
    };

    #[tokio::test]
    async fn test_order() -> anyhow::Result<()> {
        let mut client = create_client().await;
        client.get_access_token().await?;

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
                create_order,
                HeaderParams {
                    request_id: Some(ref_id.clone()),
                    ..Default::default()
                },
            )
            .await?;

        assert_ne!(order_created.id, "");
        assert_eq!(order_created.status, OrderStatus::Created);
        assert_eq!(order_created.links.len(), 4);

        let show_order = ShowOrderDetails::new(&order_created.id);

        let show_order_result = client
            .execute_ext(
                show_order,
                HeaderParams {
                    request_id: Some(ref_id.clone()),
                    ..Default::default()
                },
            )
            .await?;

        assert_eq!(order_created.id, show_order_result.id);
        assert_eq!(order_created.status, show_order_result.status);

        Ok(())
    }
}
