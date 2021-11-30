use crate::dnsimple::{Client, DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, RequestOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    pub id: u64,
    pub account_id: u64,
    pub label: String,
    pub first_name: String,
    pub last_name: String,
    pub job_title: String,
    pub organization_name: String,
    pub email: String,
    pub phone: String,
    pub fax: String,
    pub address1: String,
    pub address2: String,
    pub city: String,
    pub state_province: String,
    pub postal_code: String,
    pub country: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContactPayload {
    pub label: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub job_title: Option<String>,
    pub organization_name: Option<String>,
    pub email: String,
    pub phone: String,
    pub fax: Option<String>,
    pub address1: String,
    pub address2: Option<String>,
    pub city: String,
    pub state_province: String,
    pub postal_code: String,
    pub country: String,
}

struct ContactsEndpoint;

impl Endpoint for ContactsEndpoint {
    type Output = Vec<Contact>;
}

struct ContactEndpoint;

impl Endpoint for ContactEndpoint {
    type Output = Contact;
}

pub struct Contacts<'a> {
    pub client: &'a Client
}

impl Contacts<'_> {

    /// Lists the contacts in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    pub fn list_contacts(&self, account_id: u64, options: Option<RequestOptions>) -> Result<DNSimpleResponse<Vec<Contact>>, String> {
        let path = format!("/{}/contacts", account_id);

        self.client.get::<ContactsEndpoint>(&*path, options)
    }

    /// Create a contact in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `payload`: The `ContactPayload` with the information needed to create the contact
    pub fn create_contact(&self, account_id: u64, payload: ContactPayload) -> Result<DNSimpleResponse<Contact>, String> {
        let path = format!("/{}/contacts", account_id);

        self.client.post::<ContactEndpoint>(&*path, serde_json::to_value(payload).unwrap())
    }

    /// Retrieve a contact
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `contact`: The contact id
    pub fn get_contact(&self, account_id: u64, contact: u64) -> Result<DNSimpleResponse<Contact>, String> {
        let path = format!("/{}/contacts/{}", account_id, contact);

        self.client.get::<ContactEndpoint>(&*path, None)
    }

    /// Update a contact
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `contact`: The contact id
    /// `payload`: The `ContactPayload` with the information needed to update the contact
    pub fn update_contact(&self, account_id: u64, contact: u64, payload: ContactPayload) -> Result<DNSimpleResponse<Contact>, String> {
        let path = format!("/{}/contacts/{}", account_id, contact);

        self.client.patch::<ContactEndpoint>(&*path, serde_json::to_value(payload).unwrap())
    }

    /// Delete a contact
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `contact`: The contact id
    pub fn delete_contact(&self, account_id: u64, contact: u64) -> DNSimpleEmptyResponse {
        let path = format!("/{}/contacts/{}", account_id, contact);

        self.client.delete(&*path)
    }
}