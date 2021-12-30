use crate::common::setup_mock_for;
mod common;

#[test]
fn test_enable_dnssec() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/dnssec",
        "enableDnssec/success",
        "POST",
    );
    let client = setup.0;
    let account_id = 1385 as u64;
    let domain = "example.com";

    let response = client.domains().enable_dnssec(account_id, domain).unwrap();
    let dnssec = response.data.unwrap();

    assert_eq!(response.status, 201);

    assert_eq!(true, dnssec.enabled);
    assert_eq!("2017-03-03T13:49:58Z", dnssec.created_at);
    assert_eq!("2017-03-03T13:49:58Z", dnssec.updated_at);
}

#[test]
fn test_disable_dnssec() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/dnssec",
        "disableDnssec/success",
        "DELETE",
    );
    let client = setup.0;
    let account_id = 1385 as u64;
    let domain = "example.com";

    let response = client.domains().disable_dnssec(account_id, domain);

    assert!(response.is_ok());
    assert_eq!(204, response.unwrap().status);
}

#[test]
fn test_dnssec_status() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/dnssec",
        "getDnssec/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1385 as u64;
    let domain = "example.com";

    let response = client.domains().get_dnssec(account_id, domain).unwrap();
    let dnssec = response.data.unwrap();

    assert_eq!(response.status, 200);

    assert_eq!(true, dnssec.enabled);
    assert_eq!("2017-02-03T17:43:22Z", dnssec.created_at);
    assert_eq!("2017-02-03T17:43:22Z", dnssec.updated_at);
}
