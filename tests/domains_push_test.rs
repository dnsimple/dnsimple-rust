use dnsimple_rust::dnsimple::domains_push::InitiatePushPayload;
use crate::common::setup_mock_for;
mod common;

#[test]
fn test_initiate_push_test() {
    let setup = setup_mock_for("/1385/domains/target-account.test/pushes", "initiatePush/success", "POST");
    let client = setup.0;
    let account_id = 1385 as u64;
    let domain = "target-account.test";
    let payload = InitiatePushPayload {
        new_account_email: String::from("admin@target-account.test"),
    };

    let push = client.domains().initiate_push(account_id, domain, payload).unwrap().data.unwrap();

    assert_eq!(1, push.id);
    assert_eq!(100, push.domain_id);
    assert_eq!(None, push.contact_id);
    assert_eq!(2020, push.account_id);
    assert_eq!("2016-08-11T10:16:03Z", push.created_at);
    assert_eq!("2016-08-11T10:16:03Z", push.updated_at);
    assert_eq!(None, push.accepted_at);
}

#[test]
fn test_list_pushes() {
    let setup = setup_mock_for("/1385/domains/pushes", "listPushes/success", "GET");
    let client = setup.0;
    let account_id = 1385 as u64;

    let response = client.domains().list_pushes(account_id, None).unwrap();
    let domain_pushes_list = response.data.unwrap();

    assert_eq!(2, domain_pushes_list.len());
}

#[test]
fn test_accept_push() {
    let setup = setup_mock_for("/1385/domains/pushes/42", "acceptPush/success", "POST");
    let client = setup.0;
    let account_id = 1385 as u64;
    let push_id = 42;

    let response = client.domains().accept_push(account_id, push_id);

    assert_eq!(response.status, 204);
}
#[test]
fn test_reject_push() {
    let setup = setup_mock_for("/1385/domains/pushes/42", "rejectPush/success", "DELETE");
    let client = setup.0;
    let account_id = 1385 as u64;
    let push_id = 42;

    let response = client.domains().reject_push(account_id, push_id);

    assert_eq!(response.status, 204);
}
