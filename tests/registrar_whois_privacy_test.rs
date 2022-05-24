use crate::common::setup_mock_for;
mod common;

#[test]
fn get_whois_privacy_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/whois_privacy",
        "getWhoisPrivacy/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let whois_privacy = client
        .registrar()
        .get_whois_privacy(account_id, String::from(domain))
        .unwrap()
        .data
        .unwrap();

    assert_eq!(1, whois_privacy.id);
    assert_eq!(2, whois_privacy.domain_id);
    assert_eq!("2017-02-13", whois_privacy.expires_on.unwrap());
    assert!(whois_privacy.enabled.unwrap());
    assert_eq!("2016-02-13T14:34:50Z", whois_privacy.created_at);
    assert_eq!("2016-02-13T14:34:52Z", whois_privacy.updated_at);
}

#[test]
fn enable_whois_privacy_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/whois_privacy",
        "enableWhoisPrivacy/success",
        "PUT",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .enable_whois_privacy(account_id, String::from(domain))
        .unwrap();

    assert_eq!(200, response.status);

    let whois_privacy = response.data.unwrap();

    assert_eq!(1, whois_privacy.id);
    assert_eq!(2, whois_privacy.domain_id);
    assert_eq!("2017-02-13", whois_privacy.expires_on.unwrap());
    assert!(whois_privacy.enabled.unwrap());
    assert_eq!("2016-02-13T14:34:50Z", whois_privacy.created_at);
    assert_eq!("2016-02-13T14:36:48Z", whois_privacy.updated_at);
}

#[test]
fn enable_whois_privacy_purchased_and_enabled_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/whois_privacy",
        "enableWhoisPrivacy/created",
        "PUT",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .enable_whois_privacy(account_id, String::from(domain))
        .unwrap();

    assert_eq!(201, response.status);
}

#[test]
fn disable_whois_privacy_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/whois_privacy",
        "disableWhoisPrivacy/success",
        "DELETE",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .disable_whois_privacy(account_id, String::from(domain))
        .unwrap();

    assert_eq!(200, response.status);

    let whois_privacy = response.data.unwrap();

    assert_eq!(1, whois_privacy.id);
    assert_eq!(2, whois_privacy.domain_id);
    assert_eq!("2017-02-13", whois_privacy.expires_on.unwrap());
    assert!(!whois_privacy.enabled.unwrap());
    assert_eq!("2016-02-13T14:34:50Z", whois_privacy.created_at);
    assert_eq!("2016-02-13T14:36:38Z", whois_privacy.updated_at);
}

#[test]
fn renew_whois_privacy_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/whois_privacy",
        "renewWhoisPrivacy/success",
        "POST",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .renew_whois_privacy(account_id, String::from(domain))
        .unwrap();

    assert_eq!(201, response.status);

    let whois_privacy_renewal = response.data.unwrap();

    assert_eq!(1, whois_privacy_renewal.id);
    assert_eq!(100, whois_privacy_renewal.domain_id);
    assert_eq!(999, whois_privacy_renewal.whois_privacy_id);
    assert_eq!("new", whois_privacy_renewal.state);
    assert_eq!("2020-01-10", whois_privacy_renewal.expires_on);
    assert!(whois_privacy_renewal.enabled);
    assert_eq!("2019-01-10T12:12:48Z", whois_privacy_renewal.created_at);
    assert_eq!("2019-01-10T12:12:48Z", whois_privacy_renewal.updated_at);
}

#[test]
fn renew_whois_privacy_duplicated_order_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/whois_privacy",
        "renewWhoisPrivacy/whois-privacy-duplicated-order",
        "POST",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .renew_whois_privacy(account_id, String::from(domain));
    let errors = response.unwrap_err();

    assert_eq!("Message: \"The whois privacy for example.com has just been renewed, a new renewal cannot be started at this time\"", errors.to_string());
}

#[test]
fn renew_whois_privacy_not_found_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/whois_privacy",
        "renewWhoisPrivacy/whois-privacy-not-found",
        "POST",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .renew_whois_privacy(account_id, String::from(domain));
    let errors = response.unwrap_err();

    assert_eq!(
        "Message: \"WHOIS privacy not found for example.com\"",
        errors.to_string()
    );
}
