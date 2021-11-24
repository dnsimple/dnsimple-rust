use dnsimple_rust::dnsimple::Paginate;
use crate::common::setup_mock_for;
mod common;

#[test]
fn test_list_pushes() {
    let setup = setup_mock_for("/1385/domains/pushes", "listPushes/success", "GET");
    let client = setup.0;
    let account_id = 1385 as u64;

    let paginate = Paginate{ per_page: 0, page: 0 };

    let response = client.domains().list_pushes(account_id, paginate).unwrap();
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
