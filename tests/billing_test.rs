use crate::common::setup_mock_for;

mod common;

#[tokio::test]
async fn test_list_charges() {
    let setup = setup_mock_for("/1010/billing/charges", "listCharges/success", "GET").await;
    let client = setup.0;

    let response = client.billing().list_charges(1010, None).await.unwrap();
    let charges = response.data.unwrap();

    assert_eq!(4, charges.len());

    let charge = charges.first().unwrap();
    assert_eq!("2023-08-17T05:53:36Z", charge.invoiced_at);
    assert_eq!("14.50", charge.total_amount);
    assert_eq!("0.00", charge.balance_amount);
    assert_eq!("1-2", charge.reference);
    assert_eq!("collected", charge.state);
    assert_eq!(1, charge.items.len());

    let item = charge.items.first().unwrap();
    assert_eq!("Register bubble-registered.com", item.description);
    assert_eq!("14.50", item.amount);
    assert_eq!(Some(1), item.product_id);
    assert_eq!("domain-registration", item.product_type);
    assert_eq!(
        Some(String::from("bubble-registered.com")),
        item.product_reference
    );

    let manual_item = charges[2].items.first().unwrap();
    assert_eq!("manual", manual_item.product_type);
    assert_eq!(None, manual_item.product_id);
    assert_eq!(None, manual_item.product_reference);

    let pagination = response.pagination.unwrap();
    assert_eq!(1, pagination.current_page);
    assert_eq!(4, pagination.total_entries);
}

#[tokio::test]
async fn test_list_charges_bad_filter() {
    let setup = setup_mock_for(
        "/1010/billing/charges",
        "listCharges/fail-400-bad-filter",
        "GET",
    )
    .await;
    let client = setup.0;

    let error = client.billing().list_charges(1010, None).await.unwrap_err();

    assert_eq!(
        "Invalid date format must be ISO8601 (YYYY-MM-DD)",
        error.to_string()
    );
}
