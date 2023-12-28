use crate::common::setup_mock_for;
mod common;

#[test]
fn enable_domain_transfer_lock_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/transfer_lock",
        "enableDomainTransferLock/success",
        "POST",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .enable_domain_transfer_lock(account_id, String::from(domain))
        .unwrap();
    let transfer_lock = response.data.unwrap();

    assert_eq!(response.status, 201);

    assert!(transfer_lock.enabled);
}

#[test]
fn disable_domain_transfer_lock_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/transfer_lock",
        "disableDomainTransferLock/success",
        "DELETE",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .disable_domain_transfer_lock(account_id, String::from(domain))
        .unwrap();
    let transfer_lock = response.data.unwrap();

    assert_eq!(response.status, 200);

    assert!(!transfer_lock.enabled);
}

#[test]
fn get_domain_transfer_lock_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/transfer_lock",
        "getDomainTransferLock/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .get_domain_transfer_lock(account_id, String::from(domain))
        .unwrap();
    let transfer_lock = response.data.unwrap();

    assert_eq!(response.status, 200);

    assert!(transfer_lock.enabled);
}
