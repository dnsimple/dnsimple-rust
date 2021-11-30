use std::collections::HashMap;
use dnsimple_rust::dnsimple::{Filters, Paginate, Sort};
use crate::common::setup_mock_for;
mod common;

#[test]
fn list_domains_test() {
    let setup = setup_mock_for("/1385/domains", "listDomains/success", "GET");
    let client = setup.0;
    let account_id = 1385;

    let filters = Filters::new(HashMap::new());
    let sort = Sort::new(String::from(""));
    let paginate = Paginate{ per_page: 0, page: 0 };

    let domains_data = client.domains().list_domains(account_id, filters, sort, paginate).unwrap();
    let domains = domains_data.data.unwrap();

    assert_eq!(2, domains.len());

    let first_domain = domains.first().unwrap();
    assert_eq!(181984, first_domain.id);
    assert_eq!(account_id, first_domain.account_id);
    assert_eq!(2715, first_domain.registrant_id.unwrap());
    assert_eq!("example-alpha.com", first_domain.name);
    assert_eq!("example-alpha.com", first_domain.unicode_name);
    assert_eq!("registered", first_domain.state);
    assert_eq!(false, first_domain.auto_renew);
    assert_eq!(false, first_domain.private_whois);
    assert_eq!("2021-06-05", first_domain.expires_on.as_ref().unwrap());
    assert_eq!("2021-06-05T02:15:00Z", first_domain.expires_at.as_ref().unwrap());
    assert_eq!("2020-06-04T19:15:14Z", first_domain.created_at);
    assert_eq!("2020-06-04T19:15:21Z", first_domain.updated_at);
}

#[test]
fn create_domain_test() {
    let setup = setup_mock_for("/1385/domains", "createDomain/created", "POST");
    let client = setup.0;
    let account_id = 1385;
    let domain_name = String::from("example-beta.com");

    let domain = client.domains().create_domain(account_id, domain_name).unwrap().data.unwrap();

    assert_eq!(domain.id, 181985);
    assert_eq!(domain.account_id, account_id);
    assert_eq!(domain.registrant_id, None);
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

    let domain = client.domains().get_domain(account_id, domain_id).unwrap().data.unwrap();

    assert_eq!(domain_id, domain.id);
    assert_eq!(account_id, domain.account_id);
    assert_eq!(2715, domain.registrant_id.unwrap());
    assert_eq!("example-alpha.com", domain.name);
    assert_eq!("example-alpha.com", domain.unicode_name);
    assert_eq!("registered", domain.state);
    assert_eq!(false, domain.auto_renew);
    assert_eq!(false, domain.private_whois);
    assert_eq!("2021-06-05", domain.expires_on.unwrap());
    assert_eq!("2021-06-05T02:15:00Z", domain.expires_at.unwrap());
    assert_eq!("2020-06-04T19:15:14Z", domain.created_at);
    assert_eq!("2020-06-04T19:15:21Z", domain.updated_at);
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