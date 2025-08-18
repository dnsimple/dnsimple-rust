use crate::common::setup_mock_for;
use dnsimple::dnsimple::domains_email_forwards::EmailForwardPayload;
mod common;

#[test]
fn test_list_email_forwards() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/email_forwards",
        "listEmailForwards/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1385_u64;
    let domain = "example.com";

    let response = client
        .domains()
        .list_email_forwards(account_id, domain, None)
        .unwrap();
    let email_forwards_list = response.data.unwrap();

    assert_eq!(1, email_forwards_list.len());

    let email_forwards = email_forwards_list.first().unwrap();

    assert_eq!(24809, email_forwards.id);
    assert_eq!(235146, email_forwards.domain_id);
    assert_eq!(".*@a-domain.com", email_forwards.from);
    assert_eq!("jane.smith@example.com", email_forwards.to);
    assert_eq!("2017-05-25T19:23:16Z", email_forwards.created_at);
    assert_eq!("2017-05-25T19:23:16Z", email_forwards.updated_at);
    assert_eq!(true, email_forwards.active);
}

#[allow(deprecated)]
#[test]
fn test_create_email_forward() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/email_forwards",
        "createEmailForward/created",
        "POST",
    );
    let client = setup.0;
    let account_id = 1385_u64;
    let domain = "example.com";
    let payload = EmailForwardPayload {
        alias_name: String::from("example@dnsimple.xyz"),
        destination_email: String::from("example@example.com"),
    };

    let record = client
        .domains()
        .create_email_forward(account_id, domain, payload)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(41872, record.id);
    assert_eq!(235146, record.domain_id);
    assert_eq!("example@dnsimple.xyz", record.alias_email);
    assert_eq!("example@example.com", record.destination_email);
    assert_eq!("2021-01-25T13:54:40Z", record.created_at);
    assert_eq!("2021-01-25T13:54:40Z", record.updated_at);
    assert_eq!("example@dnsimple.xyz", record.from);
    assert_eq!("example@example.com", record.to);
    assert_eq!(true, record.active);
}

#[allow(deprecated)]
#[test]
fn test_get_email_forward() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/email_forwards/41872",
        "getEmailForward/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1385_u64;
    let domain = "example.com";
    let email_forward = 41872;

    let record = client
        .domains()
        .get_email_forward(account_id, domain, email_forward)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(41872, record.id);
    assert_eq!(235146, record.domain_id);
    assert_eq!("example@dnsimple.xyz", record.alias_email);
    assert_eq!("example@example.com", record.destination_email);
    assert_eq!("2021-01-25T13:54:40Z", record.created_at);
    assert_eq!("2021-01-25T13:54:40Z", record.updated_at);
    assert_eq!("example@dnsimple.xyz", record.from);
    assert_eq!("example@example.com", record.to);
    assert_eq!(true, record.active);
}

#[test]
fn test_delete_email_forward() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/email_forwards/41872",
        "deleteEmailForward/success",
        "DELETE",
    );
    let client = setup.0;
    let account_id = 1385_u64;
    let domain = "example.com";
    let email_forward = 41872;

    let response = client
        .domains()
        .delete_email_forward(account_id, domain, email_forward);

    assert!(response.is_ok());
    assert_eq!(204, response.unwrap().status);
}
