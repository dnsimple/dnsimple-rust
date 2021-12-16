use crate::common::setup_mock_for;
mod common;

#[test]
fn get_domain_delegation_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/delegation",
        "getDomainDelegation/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let delegation = client
        .registrar()
        .get_domain_delegation(account_id, String::from(domain))
        .unwrap()
        .data
        .unwrap();

    assert_eq!(4, delegation.len());

    for (position, value) in delegation.iter().enumerate() {
        let number = position + 1;
        let should_eq = format!("ns{}.dnsimple.com", number);
        assert_eq!(&should_eq, value);
    }
}

#[test]
fn get_empty_domain_delegation_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/delegation",
        "getDomainDelegation/success-empty",
        "GET",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let delegation = client
        .registrar()
        .get_domain_delegation(account_id, String::from(domain))
        .unwrap()
        .data
        .unwrap();

    assert!(delegation.is_empty());
}

#[test]
fn change_domain_delegation_test() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/delegation",
        "changeDomainDelegation/success",
        "PUT",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";
    let server_names = vec![
        "ns1.dnsimple.com",
        "ns2.dnsimple.com",
        "ns3.dnsimple.com",
        "ns4.dnsimple.com",
    ];

    let delegation_change = client
        .registrar()
        .change_domain_delegation(account_id, String::from(domain), server_names)
        .unwrap()
        .data
        .unwrap();

    for (position, value) in delegation_change.iter().enumerate() {
        let number = position + 1;
        let should_eq = format!("ns{}.dnsimple.com", number);
        assert_eq!(&should_eq, value);
    }
}

#[test]
fn change_domain_delegation_to_vanity() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/delegation/vanity",
        "changeDomainDelegationToVanity/success",
        "PUT",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";
    let server_names = vec!["ns1.example.com", "ns2.example.com"];

    let vanity_servers = client
        .registrar()
        .change_domain_delegation_to_vanity(account_id, String::from(domain), server_names)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(2, vanity_servers.len());

    let vanity_server = vanity_servers.first().unwrap();

    assert_eq!(1, vanity_server.id);
    assert_eq!("ns1.example.com", vanity_server.name);
    assert_eq!("127.0.0.1", vanity_server.ipv4);
    assert_eq!("::1", vanity_server.ipv6);
    assert_eq!("2016-07-11T09:40:19Z", vanity_server.created_at);
    assert_eq!("2016-07-11T09:40:19Z", vanity_server.updated_at);
}

#[test]
fn change_domain_delegation_from_vanity() {
    let setup = setup_mock_for(
        "/1385/registrar/domains/example.com/delegation/vanity",
        "changeDomainDelegationFromVanity/success",
        "DELETE",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .registrar()
        .change_domain_delegation_from_vanity(account_id, String::from(domain));

    assert_eq!(204, response.status);
}
