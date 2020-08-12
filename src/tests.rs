use crate::{
    orders::*,
    Client, HeaderParams, Prefer,
};
use std::env;

#[tokio::test]
async fn it_works() {
    dotenv::dotenv().ok();
    let clientid = env::var("PAYPAL_CLIENTID").unwrap();
    let secret = env::var("PAYPAL_SECRET").unwrap();

    let mut client = Client::new(clientid, secret, true);

    assert_eq!(client.get_access_token().await.is_err(), false, "should not error");

    let order = OrderPayload::new(Intent::Authorize, vec![PurchaseUnit::new(Amount::new("EUR", "10.0"))]);

    let ref_id = format!(
        "TEST-{:?}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    let order_created = client
        .create_order(
            order,
            HeaderParams {
                prefer: Some(Prefer::Representation),
                request_id: Some(ref_id.clone()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    assert!(order_created.id != "", "order id is not empty");
    assert_eq!(order_created.status, OrderStatus::Created, "order status is created");
    assert_eq!(order_created.links.len(), 4, "order links exist");

    client
        .update_order(
            order_created.id,
            Some(Intent::Capture),
            Some(order_created.purchase_units.expect("to exist")),
        )
        .await
        .unwrap();
}
