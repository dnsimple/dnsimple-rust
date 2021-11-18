use crate::common::setup_mock_for;
mod common;

#[test]
fn list_domains_test() {
    let setup = setup_mock_for("/1385/domains", "listDomains/success", "GET");
    let client = setup.0;
    let account_id = 1385;

    let domains_response = client.domains().list_domains(account_id).data;

    match domains_response {
        None => panic!("We should have a payload here"),
        Some(domains_data) => match domains_data.data {
            None => panic!("There should be a list of domains here!"),
            Some(domains) => {
                assert_eq!(domains.len(), 2);

                let first_domain = domains.first().unwrap();

                assert_eq!(first_domain.id, 181984);
                assert_eq!(first_domain.account_id, account_id);
                assert_eq!(first_domain.registrant_id.unwrap(), 2715);
                assert_eq!(first_domain.name, "example-alpha.com");
                assert_eq!(first_domain.unicode_name, "example-alpha.com");
                assert_eq!(first_domain.state, "registered");
                assert_eq!(first_domain.auto_renew, false);
                assert_eq!(first_domain.private_whois, false);
                assert_eq!(first_domain.expires_on.as_ref().unwrap(), "2021-06-05");
                assert_eq!(first_domain.expires_at.as_ref().unwrap(), "2021-06-05T02:15:00Z");
                assert_eq!(first_domain.created_at, "2020-06-04T19:15:14Z");
                assert_eq!(first_domain.updated_at, "2020-06-04T19:15:21Z");
            }
        }
    }
}

#[test]
fn create_domain_test() {
    let setup = setup_mock_for("/1385/domains", "createDomain/created", "POST");
    let client = setup.0;
    let account_id = 1385;
    let domain_name = String::from("example-beta.com");

    let domain = client.domains().create_domain(account_id, domain_name).data.unwrap().data;

    assert_eq!(domain.id, 181985);
    assert_eq!(domain.account_id, account_id);
    assert!(domain.registrant_id == None);
    assert_eq!(domain.name, "example-beta.com");
    assert_eq!(domain.unicode_name, "example-beta.com");
    assert_eq!(domain.state, "hosted");
    assert_eq!(domain.auto_renew, false);
    assert_eq!(domain.private_whois, false);
    assert_eq!(domain.expires_on, None);
    assert_eq!(domain.expires_at, None);
    assert_eq!(domain.created_at, "2020-06-04T19:47:05Z");
    assert_eq!(domain.updated_at, "2020-06-04T19:47:05Z");
}

#[test]
fn test_get_domain() {
    let setup = setup_mock_for("/1385/domains/181984", "getDomain/success", "GET");
    let client = setup.0;
    let account_id = 1385 as u64;
    let domain_id = 181984 as u64;

    let domain = client.domains().get_domain(account_id, domain_id).data.unwrap().data;

    assert_eq!(domain.id, domain_id);
    assert_eq!(domain.account_id, account_id);
    assert_eq!(domain.registrant_id.unwrap(), 2715);
    assert_eq!(domain.name, "example-alpha.com");
    assert_eq!(domain.unicode_name, "example-alpha.com");
    assert_eq!(domain.state, "registered");
    assert_eq!(domain.auto_renew, false);
    assert_eq!(domain.private_whois, false);
    assert_eq!(domain.expires_on.unwrap(), "2021-06-05");
    assert_eq!(domain.expires_at.unwrap(), "2021-06-05T02:15:00Z");
    assert_eq!(domain.created_at, "2020-06-04T19:15:14Z");
    assert_eq!(domain.updated_at, "2020-06-04T19:15:21Z");
}

#[test]
fn test_delete_domain() {
    let setup = setup_mock_for("/1385/domains/181984", "deleteDomain/success", "DELETE");
    let client = setup.0;
    let account_id = 1385 as u64;
    let domain_id = 181984 as u64;

    let response = client.domains().delete_domain(account_id, domain_id);

    assert_eq!(response.status, 204);
}

#[test]
fn test_list_collaborators() {
    let setup = setup_mock_for("/1385/domains/1/collaborators", "listCollaborators/success", "GET");
    let client = setup.0;
    let account_id = 1385 as u64;
    let domain_id = 1 as u64;

    let response = client.domains().list_collaborators(account_id, domain_id).data;
    
    match response {
        None => panic!("We should have a payload here"),
        Some(collaborators_data) => match collaborators_data.data {
            None => panic!("There should be a list of collaborators here!"),
            Some(collaborators) => {
                assert_eq!(collaborators.len(), 2);

                let first_collaborator = collaborators.first().unwrap();
                let second_collaborator = collaborators.last().unwrap();

                assert_eq!(first_collaborator.id, 100);
                assert_eq!(first_collaborator.domain_id, domain_id);
                assert_eq!(first_collaborator.domain_name, "example.com");
                assert_eq!(first_collaborator.user_id.unwrap(), 999);
                assert_eq!(second_collaborator.user_id, None);
                assert_eq!(first_collaborator.user_email, "existing-user@example.com");
                assert_eq!(second_collaborator.user_email, "invited-user@example.com");
                assert_eq!(first_collaborator.invitation, false);
                assert_eq!(second_collaborator.invitation, true);
                assert_eq!(first_collaborator.created_at, "2016-10-07T08:53:41Z");
                assert_eq!(first_collaborator.updated_at, "2016-10-07T08:53:41Z");
                assert_eq!(first_collaborator.accepted_at.as_ref().unwrap(), "2016-10-07T08:53:41Z");
                assert_eq!(second_collaborator.accepted_at, None);
            }
        }
    }
}

#[test]
fn test_add_collaborator_success() {
    let setup = setup_mock_for("/1385/domains/1/collaborators", "addCollaborator/success", "POST");
    let client = setup.0;
    let account_id = 1385 as u64;
    let domain_id = 1 as u64;
    let collaborator_email = "existing-user@example.com".to_string();
    let collaborator = client.domains().add_collaborator(account_id, domain_id, collaborator_email).data.unwrap().data;

    assert_eq!(collaborator.id, 100);
    assert_eq!(collaborator.domain_id, domain_id);
    assert_eq!(collaborator.domain_name, "example.com");
    assert_eq!(collaborator.user_id.unwrap(), 999);
    assert_eq!(collaborator.user_email, "existing-user@example.com");
    assert_eq!(collaborator.invitation, false);
    assert_eq!(collaborator.created_at, "2016-10-07T08:53:41Z");
    assert_eq!(collaborator.updated_at, "2016-10-07T08:53:41Z");
    assert_eq!(collaborator.accepted_at.as_ref().unwrap(), "2016-10-07T08:53:41Z");
}

#[test]
fn test_add_collaborator_invite_success() {
    let setup = setup_mock_for("/1385/domains/1/collaborators", "addCollaborator/invite-success", "POST");
    let client = setup.0;
    let account_id = 1385 as u64;
    let domain_id = 1 as u64;
    let collaborator_email = "invited-user@example.com".to_string();

    let collaborator = client.domains().add_collaborator(account_id, domain_id, collaborator_email).data.unwrap().data;

    assert_eq!(collaborator.id, 101);
    assert_eq!(collaborator.domain_id, domain_id);
    assert_eq!(collaborator.domain_name, "example.com");
    assert_eq!(collaborator.user_id, None);
    assert_eq!(collaborator.user_email, "invited-user@example.com");
    assert_eq!(collaborator.invitation, true);
    assert_eq!(collaborator.created_at, "2016-10-07T08:51:12Z");
    assert_eq!(collaborator.updated_at, "2016-10-07T08:51:12Z");
    assert_eq!(collaborator.accepted_at.as_ref(), None);
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
