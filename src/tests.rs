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

    let order = orders::OrderPayload::new(
        orders::Intent::Authorize,
        vec![orders::PurchaseUnit::new(orders::Amount::new(
            "EUR", "10.0",
        ))],
    );

    let ref_id = format!("TEST-{:?}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());

    let order_created = client.create_order(order, HeaderParams {
        prefer: Some(Prefer::Representation),
        request_id: Some(ref_id.clone()),
        ..Default::default()
    }).await.unwrap();
    
    assert!(order_created.id != "", "order id is not empty");
    assert_eq!(order_created.status, orders::OrderStatus::Created, "order status is created");
    assert_eq!(order_created.links.len(), 4, "order links exist");

    client.update_order(order_created.id, Some(orders::Intent::Capture), Some(order_created.purchase_units.expect("to exist"))).await.unwrap();
}
