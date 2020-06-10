# paypal-rs 
![Rust](https://github.com/edg-l/paypal-rs/workflows/Rust/badge.svg)
![Docs](https://docs.rs/paypal-rs/badge.svg)

A rust library that wraps the [paypal api](https://developer.paypal.com/docs/api) asynchronously in a strongly typed manner.

Crate: https://crates.io/crates/paypal-rs

Documentation: https://docs.rs/paypal-rs

Currently in early development.

## Example

```rust
use paypal_rs::{
    Client,
    HeaderParams,
    Prefer,
    orders::{OrderPayload, Intent, PurchaseUnit, Amount}
};

#[tokio::main]
async fn main() {
    let clientid = std::env::var("PAYPAL_CLIENTID").unwrap();
    let secret = std::env::var("PAYPAL_SECRET").unwrap();

    let mut client = Client::new(clientid.as_str(), secret.as_str(), true);

    client.get_access_token().await.unwrap();

    let order_payload = OrderPayload::new(
        Intent::Authorize,
        vec![PurchaseUnit::new(Amount::new(
            "EUR", "10.0",
        ))],
    );

    let order = client
        .create_order(
            order_payload,
            HeaderParams {
                prefer: Some(Prefer::Representation),
                ..Default::default()
            },
        )
        .await
        .unwrap();
}
```

## Testing
You need the enviroment variables PAYPAL_CLIENTID and PAYPAL_SECRET to be set.

`cargo test --lib`

## Roadmap

- [x] Orders API - 0.1.0
- - [x] Create order
- - [x] Update order
- - [x] Show order details
- - [x] Authorize payment for order
- - [x] Capture payment for order
- [ ] Invoicing API - 0.2.0
- [ ] Payments API - 0.3.0
- [ ] Tracking API - 0.4.0
- [ ] Subscriptions API - 0.5.0
- [ ] Identity API - 0.6.0
- [ ] Disputes API - 0.7.0
- [ ] Catalog Products API - 0.8.0
- [ ] Partner Referrals API - 0.9.0
- [ ] Payouts API - 0.10.0
- [ ] Transaction Search API - 0.11.0
- [ ] Referenced Payouts API - 0.12.0
- [ ] Vault API - 0.13.0
- [ ] Webhooks Management API - 0.14.0
- [ ] Payment Experience Web Profiles API - 1.0.0