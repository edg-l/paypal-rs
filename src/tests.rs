use crate::*;
use std::env;

#[tokio::test]
async fn it_works() {
    dotenv::dotenv().ok();
    let clientid = env::var("PAYPAL_CLIENTID").unwrap();
    let secret = env::var("PAYPAL_SECRET").unwrap();

    let mut client = Client::new(clientid.as_str(), secret.as_str(), true);

    assert_eq!(
        client.get_access_token().await.is_err(),
        false,
        "should not error"
    );
    println!("{:#?}", client);

    let order = orders::OrderPayload::new(
        orders::Intent::Capture,
        vec![orders::PurchaseUnit::new(orders::Amount::new(
            "EUR", "10.0",
        ))],
    );

    let order_created = client.create_order(order, HeaderParams::default()).await.unwrap();

    println!("{:#?}", order_created);
}
