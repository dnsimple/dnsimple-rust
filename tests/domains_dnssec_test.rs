use crate::common::setup_mock_for;
mod common;

#[tokio::test]
async fn test_enable_dnssec() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/dnssec",
        "enableDnssec/success",
        "POST",
    )
    .await;
    let client = setup.0;
    let account_id = 1385_u64;
    let domain = "example.com";

    let response = client
        .domains()
        .enable_dnssec(account_id, domain)
        .await
        .unwrap();
    let dnssec = response.data.unwrap();

    assert_eq!(response.status, 201);

    assert!(dnssec.enabled);
    assert_eq!("2017-03-03T13:49:58Z", dnssec.created_at);
    assert_eq!("2017-03-03T13:49:58Z", dnssec.updated_at);
}

#[tokio::test]
async fn test_disable_dnssec() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/dnssec",
        "disableDnssec/success",
        "DELETE",
    )
    .await;
    let client = setup.0;
    let account_id = 1385_u64;
    let domain = "example.com";

    let response = client.domains().disable_dnssec(account_id, domain).await;

    assert!(response.is_ok());
    assert_eq!(204, response.unwrap().status);
}

#[tokio::test]
async fn test_dnssec_status() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/dnssec",
        "getDnssec/success",
        "GET",
    )
    .await;
    let client = setup.0;
    let account_id = 1385_u64;
    let domain = "example.com";

    let response = client
        .domains()
        .get_dnssec(account_id, domain)
        .await
        .unwrap();
    let dnssec = response.data.unwrap();

    assert_eq!(response.status, 200);

    assert!(dnssec.enabled);
    assert_eq!("2017-02-03T17:43:22Z", dnssec.created_at);
    assert_eq!("2017-02-03T17:43:22Z", dnssec.updated_at);
}
