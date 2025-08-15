use crate::common::setup_mock_for;

mod common;

#[test]
fn list_accounts_account_success() {
    let setup = setup_mock_for("/accounts", "accounts/success-account", "GET");
    let client = setup.0;

    let accounts_response = client.accounts().list_accounts();
    let accounts = accounts_response.unwrap().data.unwrap();

    assert_eq!(1, accounts.len());
    let account = accounts.first().unwrap();
    assert_eq!(123, account.id);
    assert_eq!("john@example.com", account.email);
    assert_eq!("dnsimple-personal", account.plan_identifier);
}

#[test]
fn list_accounts_user_success() {
    let setup = setup_mock_for("/accounts", "accounts/success-user", "GET");
    let client = setup.0;

    let accounts_response = client.accounts().list_accounts();
    let accounts = accounts_response.unwrap().data.unwrap();

    assert_eq!(2, accounts.len());

    let first_account = accounts.first().unwrap();
    assert_eq!(123, first_account.id);
    assert_eq!("john@example.com", first_account.email);
    assert_eq!("dnsimple-personal", first_account.plan_identifier);

    let second_account = accounts.last().unwrap();
    assert_eq!(456, second_account.id);
    assert_eq!("ops@company.com", second_account.email);
    assert_eq!("teams-v1-monthly", second_account.plan_identifier);
}
