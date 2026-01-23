use crate::common::setup_mock_for;

mod common;

#[tokio::test]
async fn whoami_success_with_account() {
    let setup = setup_mock_for("/whoami", "whoami/success-account", "GET").await;
    let client = setup.0;
    let identity = client.identity().whoami().await.unwrap();

    let account = identity.data.unwrap().account.unwrap();

    assert_eq!(1, account.id);
    assert_eq!("example-account@example.com", account.email);
    assert_eq!("teams-v1-monthly", account.plan_identifier);
}

#[tokio::test]
async fn whoami_success_with_user() {
    let setup = setup_mock_for("/whoami", "whoami/success-user", "GET").await;
    let client = setup.0;
    let identity_response = client.identity().whoami().await.unwrap().data.unwrap();

    let user = identity_response.user.unwrap();

    assert_eq!(1, user.id);
    assert_eq!("example-user@example.com", user.email);
}
