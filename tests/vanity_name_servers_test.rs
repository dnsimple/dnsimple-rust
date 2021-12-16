use crate::common::setup_mock_for;
mod common;

#[test]
fn enable_vanity_name_servers_test() {
    let setup = setup_mock_for(
        "/1010/vanity/example.com",
        "enableVanityNameServers/success",
        "PUT",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("example.com");

    let vanity_name_servers = client
        .vanity_name_servers()
        .enable_vanity_name_servers(account_id, domain)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(4, vanity_name_servers.len());

    let vanity_name_server = vanity_name_servers.first().unwrap();

    assert_eq!(1, vanity_name_server.id);
    assert_eq!("ns1.example.com", vanity_name_server.name);
    assert_eq!("127.0.0.1", vanity_name_server.ipv4);
    assert_eq!("::1", vanity_name_server.ipv6);
    assert_eq!("2016-07-14T13:22:17Z", vanity_name_server.created_at);
    assert_eq!("2016-07-14T13:22:17Z", vanity_name_server.updated_at);
}

#[test]
fn disable_vanity_name_servers_test() {
    let setup = setup_mock_for(
        "/1010/vanity/example.com",
        "disableVanityNameServers/success",
        "DELETE",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("example.com");

    let request = client
        .vanity_name_servers()
        .disable_vanity_name_servers(account_id, domain);

    assert_eq!(204, request.status);
}
