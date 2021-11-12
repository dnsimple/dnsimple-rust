use crate::common::setup_mock_for;

mod common;

#[test]
fn list_accounts_account_success() {
    let setup = setup_mock_for("/accounts", "accounts/success-account");
    let client = setup.0;

    let accounts_response = client.accounts().list_accounts().data;

    match accounts_response {
        None => panic!("We should have a payload here"),
        Some(accounts_data) => match accounts_data.data {
            None => panic!("There should be a list of accounts here"),
            Some(accounts) => {
                assert_eq!(accounts.len(), 1);

                let account = accounts.first().unwrap();
                assert_eq!(account.id, 123);
                assert_eq!(account.email, "john@example.com");
                assert_eq!(account.plan_identifier, "dnsimple-personal");
            }
        }
    }
}

#[test]
fn list_accounts_user_success() {

    let setup = setup_mock_for("/accounts", "accounts/success-user");
    let client = setup.0;

    let accounts_response = client.accounts().list_accounts().data;

    match accounts_response {
        None => panic!("We should have a payload here"),
        Some(accounts_data) => match accounts_data.data {
            None => panic!("There should be a list of accounts here"),
            Some(accounts) => {
                assert_eq!(accounts.len(), 2);

                let first_account = accounts.first().unwrap();
                assert_eq!(first_account.id, 123);
                assert_eq!(first_account.email, "john@example.com");
                assert_eq!(first_account.plan_identifier, "dnsimple-personal");

                let second_account = accounts.last().unwrap();
                assert_eq!(second_account.id, 456);
                assert_eq!(second_account.email, "ops@company.com");
                assert_eq!(second_account.plan_identifier, "dnsimple-professional");

            }
        }
    }
}