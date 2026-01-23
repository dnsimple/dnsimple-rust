use crate::common::setup_mock_for;
use dnsimple::dnsimple::registrar_registrant_changes::{
    RegistrantChangeCheckPayload, RegistrantChangePayload,
};
mod common;
use std::collections::HashMap;

#[tokio::test]
async fn get_registrant_change_test() {
    let setup = setup_mock_for(
        "/101/registrar/registrant_changes/101",
        "getRegistrantChange/success",
        "GET",
    )
    .await;
    let client = setup.0;
    let account_id = 101;
    let registrant_change_id = 101;

    let response = client
        .registrar()
        .get_registrant_change(account_id, registrant_change_id)
        .await
        .unwrap();
    let registrant_change = response.data.unwrap();

    assert_eq!(101, registrant_change.id);
    assert_eq!(101, registrant_change.account_id);
    assert_eq!(101, registrant_change.domain_id);
    assert_eq!(101, registrant_change.contact_id);
    assert_eq!("new", registrant_change.state);
    assert_eq!(
        HashMap::new(),
        registrant_change.extended_attributes.unwrap()
    );
    assert!(registrant_change.registry_owner_change);
    assert_eq!(None, registrant_change.irt_lock_lifted_by);

    assert_eq!("2017-02-03T17:43:22Z", registrant_change.created_at);
    assert_eq!("2017-02-03T17:43:22Z", registrant_change.updated_at);
}

#[tokio::test]
async fn check_registrant_change_test() {
    let setup = setup_mock_for(
        "/101/registrar/registrant_changes/check",
        "checkRegistrantChange/success",
        "POST",
    )
    .await;
    let client = setup.0;
    let account_id = 101;
    let domain_id = 101;
    let contact_id = 101;
    let payload = RegistrantChangeCheckPayload {
        domain_id,
        contact_id,
    };

    let response = client
        .registrar()
        .check_registrant_change(account_id, payload)
        .await
        .unwrap();
    let registrant_change = response.data.unwrap();

    assert_eq!(101, registrant_change.contact_id);
    assert_eq!(101, registrant_change.domain_id);
    assert_eq!(0, registrant_change.extended_attributes.unwrap().len());
    assert!(registrant_change.registry_owner_change);
}

#[tokio::test]
async fn create_registrant_change_test() {
    let setup = setup_mock_for(
        "/101/registrar/registrant_changes",
        "createRegistrantChange/success",
        "POST",
    )
    .await;
    let client = setup.0;
    let account_id = 101;
    let domain_id = 101;
    let contact_id = 101;
    let payload = RegistrantChangePayload {
        domain_id,
        contact_id,
        extended_attributes: Option::Some(HashMap::from([(
            String::from("foo"),
            String::from("bar"),
        )])),
    };

    let response = client
        .registrar()
        .create_registrant_change(account_id, payload)
        .await
        .unwrap();
    let registrant_change = response.data.unwrap();

    assert_eq!(101, registrant_change.id);
    assert_eq!(101, registrant_change.account_id);
    assert_eq!(101, registrant_change.domain_id);
    assert_eq!(101, registrant_change.contact_id);
    assert_eq!("new", registrant_change.state);
    assert_eq!(
        HashMap::new(),
        registrant_change.extended_attributes.unwrap()
    );
    assert!(registrant_change.registry_owner_change);
    assert_eq!(None, registrant_change.irt_lock_lifted_by);

    assert_eq!("2017-02-03T17:43:22Z", registrant_change.created_at);
    assert_eq!("2017-02-03T17:43:22Z", registrant_change.updated_at);
}

#[tokio::test]
async fn list_registrant_changes_test() {
    let setup = setup_mock_for(
        "/101/registrar/registrant_changes",
        "listRegistrantChanges/success",
        "GET",
    )
    .await;
    let client = setup.0;
    let account_id = 101;

    let response = client
        .registrar()
        .list_registrant_changes(account_id, None)
        .await
        .unwrap();
    let registrant_changes = response.data.unwrap();
    let pagination = response.pagination.unwrap();

    assert_eq!(1, pagination.total_entries);

    assert_eq!(1, registrant_changes.len());

    let registrant_change = &registrant_changes[0];
    assert_eq!(101, registrant_change.id);
    assert_eq!(101, registrant_change.account_id);
    assert_eq!(101, registrant_change.domain_id);
    assert_eq!(101, registrant_change.contact_id);
    assert_eq!("new", registrant_change.state);
    assert_eq!(
        &HashMap::new(),
        registrant_change.extended_attributes.as_ref().unwrap()
    );
    assert!(registrant_change.registry_owner_change);
    assert_eq!(None, registrant_change.irt_lock_lifted_by);

    assert_eq!("2017-02-03T17:43:22Z", registrant_change.created_at);
    assert_eq!("2017-02-03T17:43:22Z", registrant_change.updated_at);
}

#[tokio::test]
async fn delete_registrant_change_test() {
    let setup = setup_mock_for(
        "/101/registrar/registrant_changes/101",
        "deleteRegistrantChange/success",
        "DELETE",
    )
    .await;
    let client = setup.0;
    let account_id = 101;
    let registrant_change_id = 101;

    let response = client
        .registrar()
        .delete_registrant_change(account_id, registrant_change_id)
        .await;

    // assert!(response.is_ok());
    assert_eq!(204, response.unwrap().status);
}

#[tokio::test]
async fn delete_registrant_change_async_response_test() {
    let setup = setup_mock_for(
        "/101/registrar/registrant_changes/101",
        "deleteRegistrantChange/success_async",
        "DELETE",
    )
    .await;
    let client = setup.0;
    let account_id = 101;
    let registrant_change_id = 101;

    let response = client
        .registrar()
        .delete_registrant_change(account_id, registrant_change_id)
        .await
        .unwrap();
    let registrant_change = response.data.unwrap().unwrap();

    assert_eq!(101, registrant_change.id);
    assert_eq!(101, registrant_change.account_id);
    assert_eq!(101, registrant_change.domain_id);
    assert_eq!(101, registrant_change.contact_id);
    assert_eq!("cancelling", registrant_change.state);
    assert_eq!(
        &HashMap::new(),
        registrant_change.extended_attributes.as_ref().unwrap()
    );
    assert!(registrant_change.registry_owner_change);
    assert_eq!(None, registrant_change.irt_lock_lifted_by);

    assert_eq!("2017-02-03T17:43:22Z", registrant_change.created_at);
    assert_eq!("2017-02-03T17:43:22Z", registrant_change.updated_at);
}
