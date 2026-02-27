use crate::common::setup_mock_for;
mod common;

#[tokio::test]
async fn enable_whois_privacy_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/whois_privacy",
        "enableWhoisPrivacy/success",
        "PUT",
    )
    .await;
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .enable_whois_privacy(account_id, String::from(domain))
        .await
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

#[tokio::test]
async fn enable_whois_privacy_purchased_and_enabled_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/whois_privacy",
        "enableWhoisPrivacy/created",
        "PUT",
    )
    .await;
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .enable_whois_privacy(account_id, String::from(domain))
        .await
        .unwrap();

    assert_eq!(201, response.status);
}

#[tokio::test]
async fn disable_whois_privacy_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/whois_privacy",
        "disableWhoisPrivacy/success",
        "DELETE",
    )
    .await;
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .disable_whois_privacy(account_id, String::from(domain))
        .await
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
