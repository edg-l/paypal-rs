use paypal_rs::{Client, PaypalEnv};
use wiremock::matchers::{basic_auth, body_string, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn create_client(url: &str) -> Client {
    Client::new(
        "clientid".to_string(),
        "secret".to_string(),
        PaypalEnv::Mock(url.to_string()),
    )
}

#[tokio::test]
async fn test_auth() -> color_eyre::Result<()> {
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

    let mut client = create_client(&mock_server.uri());

    client.get_access_token().await?;

    Ok(())
}
