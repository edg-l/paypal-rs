use paypal_rs::{
    api::orders::*,
    data::{common::AddressBuilder, orders::*},
};
use paypal_rs::{Client, PaypalEnv};
use wiremock::matchers::{basic_auth, bearer_token, body_string, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn create_client(url: &str) -> Client {
    Client::new(
        "clientid".to_string(),
        "secret".to_string(),
        PaypalEnv::Mock(url.to_string()),
    )
}

#[tokio::test]
async fn test_create_order() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mock_server = MockServer::start().await;

    let access_token: serde_json::Value = serde_json::from_str(include_str!("resources/oauth_token.json")).unwrap();

    Mock::given(method("POST"))
        .and(path("/v1/oauth2/token"))
        .and(basic_auth("clientid", "secret"))
        .and(header("Content-Type", "x-www-form-urlencoded"))
        .and(body_string("grant_type=client_credentials"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&access_token))
        .mount(&mock_server)
        .await;

    let response_body: serde_json::Value =
        serde_json::from_str(include_str!("resources/create_order_response.json")).unwrap();

    Mock::given(method("POST"))
        .and(path("/v2/checkout/orders"))
        .and(bearer_token("TESTBEARERTOKEN"))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let mut client = create_client(&mock_server.uri());

    client.get_access_token().await?;

    let order = OrderPayloadBuilder::default()
        .intent(Intent::Authorize)
        .purchase_units(vec![PurchaseUnitBuilder::default()
            .reference_id("d9f80740-38f0-11e8-b467-0ed5f89f718b")
            .amount(Amount::usd("100.00"))
            .build()?])
        .payment_source(
            OrderPaymentSourceBuilder::default()
                .card(
                    PaymentCardBuilder::default()
                        .number("4111111111111111")
                        .expiry("2020-02")
                        .name("John Doe")
                        .billing_address(
                            AddressBuilder::default()
                                .address_line_1("2211 N First Street")
                                .address_line_2("17.3.160")
                                .admin_area_1("CA")
                                .admin_area_2("San Jose")
                                .postal_code("95131")
                                .country_code("US")
                                .build()?,
                        )
                        .build()?,
                )
                .stored_credential(
                    StoredCredentialBuilder::default()
                        .payment_initiator("MERCHANT")
                        .payment_type("RECURRING")
                        .usage("SUBSEQUENT")
                        .previous_network_transaction_reference(
                            TransactionReferenceBuilder::default()
                                .id("156GHJ654SFH543")
                                .network("VISA")
                                .build()?,
                        )
                        .build()?,
                )
                .build()?,
        )
        .build()?;

    let create_order = CreateOrder::new(order);

    Ok(())
}

/*

#[tokio::test]
async fn test_order2() -> anyhow::Result<()> {
    let mock_server = MockServer::start().await;


    let mut client = create_client(&mock_server.uri());
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
} */
