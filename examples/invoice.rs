use color_eyre::Result;
use paypal_rs::data::invoice::*;
use paypal_rs::{api::invoice::*, data::common::Money, PaypalEnv};
use paypal_rs::{data::common::Currency, Client};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();

    let clientid = std::env::var("PAYPAL_CLIENTID")?;
    let secret = std::env::var("PAYPAL_SECRET")?;

    let mut client = Client::new(clientid, secret, PaypalEnv::Sandbox);
    client.get_access_token().await?;

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

    let res = client.execute(&invoice).await?;

    println!("{:#?}", res);

    Ok(())
}
