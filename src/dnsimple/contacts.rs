use crate::dnsimple::{Client, DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, RequestOptions};
use crate::errors::DNSimpleError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    /// The contact ID in DNSimple.
    pub id: u64,
    /// The associated account ID.
    pub account_id: u64,
    /// The label to represent the contact.
    pub label: String,
    /// The contact first name.
    pub first_name: String,
    /// The contact last name.
    pub last_name: String,
    /// The contact's job title.
    pub job_title: String,
    /// The name of the organization in which the contact works.
    pub organization_name: String,
    /// The contact email address.
    pub email: String,
    /// The contact phone number.
    pub phone: String,
    /// The contact fax number.
    pub fax: String,
    ///  The contact street address.
    pub address1: String,
    /// Apartment or suite number.
    pub address2: String,
    /// The city name.
    pub city: String,
    /// The state or province name.
    pub state_province: String,
    /// The contact postal code.
    pub postal_code: String,
    ///  The contact country (as a 2-character country code).
    pub country: String,
    /// When the contact was created in DNSimple.
    pub created_at: String,
    /// When the contact was last updated in DNSimple.
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContactPayload {
    /// The label to represent the contact.
    pub label: Option<String>,
    /// The contact first name.
    pub first_name: String,
    /// The contact last name.
    pub last_name: String,
    /// The contact's job title.
    pub job_title: Option<String>,
    /// The name of the organization in which the contact works.
    pub organization_name: Option<String>,
    /// The contact email address.
    pub email: String,
    /// The contact phone number.
    pub phone: String,
    /// The contact fax number
    pub fax: Option<String>,
    ///  The contact street address.
    pub address1: String,
    /// Apartment or suite number.
    pub address2: Option<String>,
    /// The city name.
    pub city: String,
    /// The contact postal code.
    pub state_province: String,
    /// The contact postal code.
    pub postal_code: String,
    ///  The contact country (as a 2-character country code).
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

/// The Contacts Service handles the contacts endpoint of the DNSimple API.
///
/// See [API Documentation: contacts](https://developer.dnsimple.com/v2/contacts/)
pub struct Contacts<'a> {
    pub client: &'a Client,
}

impl Contacts<'_> {
    /// Lists the contacts in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `options`: The `RequestOptions`
    ///            - Sort: `id`, `label`, `email`
    pub fn list_contacts(
        &self,
        account_id: u64,
        options: Option<RequestOptions>,
    ) -> Result<DNSimpleResponse<Vec<Contact>>, DNSimpleError> {
        let path = format!("/{}/contacts", account_id);

        self.client.get::<ContactsEndpoint>(&path, options)
    }

    /// Create a contact in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `payload`: The `ContactPayload` with the information needed to create the contact
    pub fn create_contact(
        &self,
        account_id: u64,
        payload: ContactPayload,
    ) -> Result<DNSimpleResponse<Contact>, DNSimpleError> {
        let path = format!("/{}/contacts", account_id);

        match serde_json::to_value(payload) {
            Ok(json) => self.client.post::<ContactEndpoint>(&path, json),
            Err(_) => Err(DNSimpleError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }

    /// Retrieve a contact
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `contact`: The contact id
    pub fn get_contact(
        &self,
        account_id: u64,
        contact: u64,
    ) -> Result<DNSimpleResponse<Contact>, DNSimpleError> {
        let path = format!("/{}/contacts/{}", account_id, contact);

        self.client.get::<ContactEndpoint>(&path, None)
    }

    /// Update a contact
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `contact`: The contact id
    /// `payload`: The `ContactPayload` with the information needed to update the contact
    pub fn update_contact(
        &self,
        account_id: u64,
        contact: u64,
        payload: ContactPayload,
    ) -> Result<DNSimpleResponse<Contact>, DNSimpleError> {
        let path = format!("/{}/contacts/{}", account_id, contact);

        match serde_json::to_value(payload) {
            Ok(json) => self.client.patch::<ContactEndpoint>(&path, json),
            Err(_) => Err(DNSimpleError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }

    /// Delete a contact
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `contact`: The contact id
    pub fn delete_contact(
        &self,
        account_id: u64,
        contact: u64,
    ) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let path = format!("/{}/contacts/{}", account_id, contact);

        self.client.delete(&path)
    }
}
