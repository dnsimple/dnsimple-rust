use crate::common::setup_mock_for;
mod common;

#[tokio::test]
async fn enable_domain_auto_renewal_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/auto_renewal",
        "enableDomainAutoRenewal/success",
        "PUT",
    )
    .await;
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .enable_domain_auto_renewal(account_id, String::from(domain))
        .await;

    assert!(response.is_ok());
    assert_eq!(204, response.unwrap().status);
}

#[tokio::test]
async fn disable_domain_auto_renewal_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/auto_renewal",
        "disableDomainAutoRenewal/success",
        "DELETE",
    )
    .await;
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .disable_domain_auto_renewal(account_id, String::from(domain))
        .await;

    assert!(response.is_ok());
    assert_eq!(204, response.unwrap().status);
}
