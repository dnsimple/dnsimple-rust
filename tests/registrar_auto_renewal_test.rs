use crate::common::setup_mock_for;
mod common;

#[test]
fn enable_domain_auto_renewal_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/auto_renewal",
        "enableDomainAutoRenewal/success",
        "PUT",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .enable_domain_auto_renewal(account_id, String::from(domain));

    assert_eq!(204, response.status);
}

#[test]
fn disable_domain_auto_renewal_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/auto_renewal",
        "disableDomainAutoRenewal/success",
        "DELETE",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .disable_domain_auto_renewal(account_id, String::from(domain));

    assert_eq!(204, response.status);
}
