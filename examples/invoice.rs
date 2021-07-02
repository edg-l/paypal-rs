use paypal_rs::{common::*, errors::*, invoice::*, Client, HeaderParams, Prefer};

#[tokio::main]
async fn main() -> Result<(), ResponseError> {
    dotenv::dotenv().ok();

    let clientid = std::env::var("PAYPAL_CLIENTID").unwrap();
    let secret = std::env::var("PAYPAL_SECRET").unwrap();

    let mut client = Client::new(clientid, secret, true);

    let payload = InvoicePayload {
        detail: InvoiceDetail {
            currency_code: Currency::EUR,
            //reference: Some("deal-ref".to_owned()),
            ..Default::default()
        },
        invoicer: Some(InvoicerInfo {
            name: Some(Name {
                given_name: Some("Lucas".to_owned()),
                prefix: None,
                suffix: None,
                surname: None,
                full_name: None,
                middle_name: None,
                alternate_full_name: None,
            }),
            phones: None,
            tax_id: None,
            website: None,
            business_name: "Lucas Corp".to_owned(),
            logo_url: None,
            // needs to be a valid address...
            email_address: Some("merchant@example.com".to_owned()),
            additional_notes: None,
        }),
        items: vec![Item {
            id: None,
            name: "My item".to_owned(),
            unit_amount: Money {
                currency_code: Currency::EUR,
                value: "10.0".to_owned(),
            },
            quantity: "1".to_owned(),
            discount: None,
            item_date: None,
            description: Some("A random item".to_owned()),
            tax: Some(Tax {
                name: "Sales tax".to_owned(),
                percent: "7".to_owned(),
                amount: None,
            }),
            unit_of_measure: Some(UnitOfMeasure::Quantity),
        }],
        ..Default::default()
    };
    match client.create_draft_invoice(payload, HeaderParams::default()).await {
        Ok(r) => {
            println!("{:#?}", r);
        }
        Err(ResponseError::HttpError(e)) => {
            println!("{}", e);
        }
        Err(e) => {
            println!("{:#?}", e);
        }
    }

    // some stuff is not sent when representation is minimal.

    Ok(())
}
