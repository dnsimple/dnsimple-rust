use crate::common::setup_mock_for;
mod common;

#[tokio::test]
async fn enable_domain_transfer_lock_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/transfer_lock",
        "enableDomainTransferLock/success",
        "POST",
    )
    .await;
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .enable_domain_transfer_lock(account_id, String::from(domain))
        .await
        .unwrap();
    let transfer_lock = response.data.unwrap();

    assert_eq!(response.status, 201);

    assert!(transfer_lock.enabled);
}

#[tokio::test]
async fn disable_domain_transfer_lock_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/transfer_lock",
        "disableDomainTransferLock/success",
        "DELETE",
    )
    .await;
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .disable_domain_transfer_lock(account_id, String::from(domain))
        .await
        .unwrap();
    let transfer_lock = response.data.unwrap();

    assert_eq!(response.status, 200);

    assert!(!transfer_lock.enabled);
}

#[tokio::test]
async fn get_domain_transfer_lock_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/transfer_lock",
        "getDomainTransferLock/success",
        "GET",
    )
    .await;
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .get_domain_transfer_lock(account_id, String::from(domain))
        .await
        .unwrap();
    let transfer_lock = response.data.unwrap();

    assert_eq!(response.status, 200);

    assert!(transfer_lock.enabled);
}
