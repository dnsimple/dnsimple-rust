use dnsimple_rust::dnsimple::Paginate;
use crate::common::setup_mock_for;
mod common;

#[test]
fn test_list_collaborators() {
    let setup = setup_mock_for("/1385/domains/1/collaborators", "listCollaborators/success", "GET");
    let client = setup.0;
    let account_id = 1385 as u64;
    let domain_id = 1 as u64;

    let paginate = Paginate{ per_page: 0, page: 0 };

    let response = client.domains().list_collaborators(account_id, domain_id, paginate);
    let collaborators = response.unwrap().data.unwrap();

    assert_eq!(2, collaborators.len());
    let first_collaborator = collaborators.first().unwrap();
    let second_collaborator = collaborators.last().unwrap();

    assert_eq!(100, first_collaborator.id);
    assert_eq!(domain_id, first_collaborator.domain_id);
    assert_eq!("example.com", first_collaborator.domain_name);
    assert_eq!(999, first_collaborator.user_id.unwrap());
    assert_eq!(None, second_collaborator.user_id);
    assert_eq!("existing-user@example.com", first_collaborator.user_email);
    assert_eq!("invited-user@example.com", second_collaborator.user_email);
    assert_eq!(false, first_collaborator.invitation);
    assert_eq!(true, second_collaborator.invitation);
    assert_eq!("2016-10-07T08:53:41Z", first_collaborator.created_at);
    assert_eq!("2016-10-07T08:53:41Z", first_collaborator.updated_at);
    assert_eq!("2016-10-07T08:53:41Z", first_collaborator.accepted_at.as_ref().unwrap());
    assert_eq!(None, second_collaborator.accepted_at);
}

#[test]
fn test_add_collaborator_success() {
    let setup = setup_mock_for("/1385/domains/1/collaborators", "addCollaborator/success", "POST");
    let client = setup.0;
    let account_id = 1385 as u64;
    let domain_id = 1 as u64;
    let collaborator_email = "existing-user@example.com".to_string();
    let collaborator = client.domains().add_collaborator(account_id, domain_id, collaborator_email).unwrap().data.unwrap();

    assert_eq!(100, collaborator.id);
    assert_eq!(domain_id, collaborator.domain_id);
    assert_eq!("example.com", collaborator.domain_name);
    assert_eq!(999, collaborator.user_id.unwrap());
    assert_eq!("existing-user@example.com", collaborator.user_email);
    assert_eq!(false, collaborator.invitation);
    assert_eq!("2016-10-07T08:53:41Z", collaborator.created_at);
    assert_eq!("2016-10-07T08:53:41Z", collaborator.updated_at);
    assert_eq!("2016-10-07T08:53:41Z", collaborator.accepted_at.as_ref().unwrap());
}

#[test]
fn test_add_collaborator_invite_success() {
    let setup = setup_mock_for("/1385/domains/1/collaborators", "addCollaborator/invite-success", "post");
    let client = setup.0;
    let account_id = 1385 as u64;
    let domain_id = 1 as u64;
    let collaborator_email = "invited-user@example.com".to_string();

    let collaborator = client.domains().add_collaborator(account_id, domain_id, collaborator_email).unwrap().data.unwrap();

    assert_eq!(101, collaborator.id);
    assert_eq!(domain_id, collaborator.domain_id);
    assert_eq!("example.com", collaborator.domain_name);
    assert_eq!(None, collaborator.user_id);
    assert_eq!("invited-user@example.com", collaborator.user_email);
    assert_eq!(true, collaborator.invitation);
    assert_eq!("2016-10-07T08:51:12Z", collaborator.created_at);
    assert_eq!("2016-10-07T08:51:12Z", collaborator.updated_at);
    assert_eq!(None, collaborator.accepted_at.as_ref());
}

#[test]
fn test_remove_collaborator() {
    let setup = setup_mock_for("/1385/domains/1/collaborators/100", "removeCollaborator/success", "DELETE");
    let client = setup.0;
    let account_id = 1385 as u64;
    let domain_id = 1 as u64;

    let response = client.domains().remove_collaborator(account_id, domain_id, 100);

    assert_eq!(response.status, 204);
}
