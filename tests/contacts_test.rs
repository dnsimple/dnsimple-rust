use dnsimple::dnsimple::contacts::ContactPayload;
use crate::common::setup_mock_for;
mod common;

#[test]
fn list_contacts_test() {
    let setup = setup_mock_for("/1010/contacts", "listContacts/success", "GET");
    let client = setup.0;
    let account_id = 1010;

    let contacts = client.contacts().list_contacts(account_id, None).unwrap().data.unwrap();

    assert_eq!(2, contacts.len());

    let contact = contacts.first().unwrap();

    assert_eq!(1, contact.id);
    assert_eq!(account_id, contact.account_id);
    assert_eq!("Default", contact.label);
    assert_eq!("First", contact.first_name);
    assert_eq!("User", contact.last_name);
    assert_eq!("CEO", contact.job_title);
    assert_eq!("Awesome Company", contact.organization_name);
    assert_eq!("first@example.com", contact.email);
    assert_eq!("+18001234567", contact.phone);
    assert_eq!("+18011234567", contact.fax);
    assert_eq!("Italian Street, 10", contact.address1);
    assert_eq!("", contact.address2);
    assert_eq!("Roma", contact.city);
    assert_eq!("RM", contact.state_province);
    assert_eq!("00100", contact.postal_code);
    assert_eq!("IT", contact.country);
    assert_eq!("2013-11-08T17:23:15Z", contact.created_at);
    assert_eq!("2015-01-08T21:30:50Z", contact.updated_at);
}

#[test]
fn create_contact_test() {
    let setup = setup_mock_for("/1010/contacts", "createContact/created", "POST");
    let client = setup.0;
    let account_id = 1010;
    let payload = ContactPayload {
        label: Some(String::from("Default")),
        first_name: String::from("First"),
        last_name: String::from("User"),
        organization_name: Some(String::from("Awesome Company")),
        job_title: Some(String::from("CEO")),
        address1: String::from("Italian Street, 10"),
        address2: None,
        city: String::from("Roma"),
        state_province: String::from("RM"),
        postal_code: String::from("00100"),
        country: String::from("IT"),
        email: String::from("first@example.com"),
        phone: String::from("+18001234567"),
        fax: Some(String::from("+18011234567")),
    };

    let contact = client.contacts().create_contact(account_id, payload).unwrap().data.unwrap();

    assert_eq!(1, contact.id);
    assert_eq!(account_id, contact.account_id);
    assert_eq!("Default", contact.label);
    assert_eq!("First", contact.first_name);
    assert_eq!("User", contact.last_name);
    assert_eq!("CEO", contact.job_title);
    assert_eq!("Awesome Company", contact.organization_name);
    assert_eq!("first@example.com", contact.email);
    assert_eq!("+18001234567", contact.phone);
    assert_eq!("+18011234567", contact.fax);
    assert_eq!("Italian Street, 10", contact.address1);
    assert!(contact.address2.is_empty());
    assert_eq!("Roma", contact.city);
    assert_eq!("RM", contact.state_province);
    assert_eq!("00100", contact.postal_code);
    assert_eq!("IT", contact.country);
}

#[test]
fn get_contact_test() {
    let setup = setup_mock_for("/1010/contacts/1", "getContact/success", "GET");
    let client = setup.0;
    let account_id = 1010;
    let contact_id = 1;

    let contact = client.contacts().get_contact(account_id, contact_id).unwrap().data.unwrap();

    assert_eq!(1, contact.id);
    assert_eq!(account_id, contact.account_id);
    assert_eq!("Default", contact.label);
    assert_eq!("First", contact.first_name);
    assert_eq!("User", contact.last_name);
    assert_eq!("CEO", contact.job_title);
    assert_eq!("Awesome Company", contact.organization_name);
    assert_eq!("first@example.com", contact.email);
    assert_eq!("+18001234567", contact.phone);
    assert_eq!("+18011234567", contact.fax);
    assert_eq!("Italian Street, 10", contact.address1);
    assert_eq!("", contact.address2);
    assert_eq!("Roma", contact.city);
    assert_eq!("RM", contact.state_province);
    assert_eq!("00100", contact.postal_code);
    assert_eq!("IT", contact.country);
    assert_eq!("2016-01-19T20:50:26Z", contact.created_at);
    assert_eq!("2016-01-19T20:50:26Z", contact.updated_at);
}

#[test]
fn update_contact_test() {
    let setup = setup_mock_for("/1010/contacts/1", "updateContact/success", "PATCH");
    let client = setup.0;
    let account_id = 1010;
    let contact_id = 1;
    let payload = ContactPayload {
        label: Some(String::from("Default")),
        first_name: String::from("First"),
        last_name: String::from("User"),
        organization_name: Some(String::from("Awesome Company")),
        job_title: Some(String::from("CEO")),
        address1: String::from("Italian Street, 10"),
        address2: None,
        city: String::from("Roma"),
        state_province: String::from("RM"),
        postal_code: String::from("00100"),
        country: String::from("IT"),
        email: String::from("first@example.com"),
        phone: String::from("+18001234567"),
        fax: Some(String::from("+18011234567")),
    };

    let contact = client.contacts().update_contact(account_id, contact_id, payload).unwrap().data.unwrap();

    assert_eq!(1, contact.id);
    assert_eq!(account_id, contact.account_id);
    assert_eq!("Default", contact.label);
    assert_eq!("First", contact.first_name);
    assert_eq!("User", contact.last_name);
    assert_eq!("CEO", contact.job_title);
    assert_eq!("Awesome Company", contact.organization_name);
    assert_eq!("first@example.com", contact.email);
    assert_eq!("+18001234567", contact.phone);
    assert_eq!("+18011234567", contact.fax);
    assert_eq!("Italian Street, 10", contact.address1);
    assert!(contact.address2.is_empty());
    assert_eq!("Roma", contact.city);
    assert_eq!("RM", contact.state_province);
    assert_eq!("00100", contact.postal_code);
    assert_eq!("IT", contact.country);
}

#[test]
fn delete_contact_test() {
    let setup = setup_mock_for("/1010/contacts/1", "deleteContact/success", "DELETE");
    let client = setup.0;
    let account_id = 1010;
    let contact_id = 1;

    let response = client.contacts().delete_contact(account_id, contact_id);

    assert_eq!(204, response.status);
}
