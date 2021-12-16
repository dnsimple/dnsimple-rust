use crate::common::setup_mock_for;

mod common;

#[test]
fn whoami_success_with_account() {
    let setup = setup_mock_for("/whoami", "whoami/success-account", "GET");
    let client = setup.0;
    let identity = client.identity().whoami().unwrap();

    let account = identity.data.unwrap().account.unwrap();

    assert_eq!(1, account.id);
    assert_eq!("example-account@example.com", account.email);
    assert_eq!("dnsimple-professional", account.plan_identifier);
}

#[test]
fn whoami_success_with_user() {
    let setup = setup_mock_for("/whoami", "whoami/success-user", "GET");
    let client = setup.0;
    let identity_response = client.identity().whoami().unwrap().data.unwrap();

    let user = identity_response.user.unwrap();

    assert_eq!(1, user.id);
    assert_eq!("example-user@example.com", user.email);
}
