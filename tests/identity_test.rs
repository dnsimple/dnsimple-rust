use crate::common::setup_mock_for;

mod common;

#[test]
fn whoami_success_with_account() {
    let setup = setup_mock_for("/whoami","whoami/success-account");
    let client = setup.0;
    let identity_response = client.identity().whoami().data;

    match identity_response {
        None => panic!("We should have a payload here."),
        Some(whoami) =>  match whoami.data.account {
            None => panic!("We should have the account data here"),
            Some(account) => {
                assert_eq!(account.id, 1);
                assert_eq!(account.email, "example-account@example.com");
                assert_eq!(account.plan_identifier, "dnsimple-professional");
            }
        }
    }
}

#[test]
fn whoami_success_with_user() {
    let setup = setup_mock_for("/whoami", "whoami/success-user");
    let client = setup.0;
    let identity_response = client.identity().whoami().data;

    match identity_response {
        None => panic!("We should have a payload here."),
        Some(whoami) => match whoami.data.user {
            None => panic!("We should have the user data here"),
            Some(user) => {
                assert_eq!(user.id, 1);
                assert_eq!(user.email, "example-user@example.com");
            }
        }
    }
}