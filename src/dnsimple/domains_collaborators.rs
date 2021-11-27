use crate::dnsimple::domains::Domains;
use crate::dnsimple::{DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, Paginate, RequestOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Collaborator {
    pub id: u64,
    pub domain_id: u64,
    pub domain_name: String,
    pub user_id: Option<u64>,
    pub user_email: String,
    pub invitation: bool,
    pub created_at: String,
    pub updated_at: String,
    pub accepted_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddCollaboratorPayload {
    pub email: String,

}

struct ListCollaboratorsEndpoint;

impl Endpoint for ListCollaboratorsEndpoint {
    type Output = Vec<Collaborator>;
}

struct CollaboratorEndpoint;

impl Endpoint for CollaboratorEndpoint {
    type Output = Collaborator;
}
impl Domains<'_> {

    /// List collaborators for the domain in the account.
    ///
    /// # Examples
    /// ```no_run
    /// use dnsimple_rust::dnsimple::{Filters, new_client, Paginate, Sort};
    /// use std::collections::HashMap;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let paginate = Paginate{per_page: 0,page: 0};
    /// let collaborators = client.domains().list_collaborators(1234, 1, paginate);
    /// ```
    ///
    /// # Arguments
    /// `account_id`: The account ID
    /// `domain_id`: The ID of the domain we want to list the collaborators of
    pub fn list_collaborators(&self, account_id: u64, domain_id: u64, paginate: Paginate) -> Result<DNSimpleResponse<Vec<Collaborator>>, String> {
        let path = format!("/{}/domains/{}/collaborators", account_id, domain_id);

        self.client.get::<ListCollaboratorsEndpoint>(&*path, Option::from(RequestOptions{ filters: None, sort: None, paginate: Some(paginate)}))
    }

    // At the time of the add, a collaborator may or may not have a DNSimple account.
//
// In case the collaborator doesn't have a DNSimple account, the system will invite them to
// register to DNSimple first and then to accept the collaboration invitation.
//
// In the other case, they are automatically added to the domain as collaborator. They can
// decide to reject the invitation later.
//
// # Examples
// ```no_run
// use dnsimple_rust::dnsimple::domains::AddCollaboratorPayload;
// use dnsimple_rust::dnsimple::new_client;
//
// let client = new_client(true, String::from("AUTH_TOKEN"));
// let email = "existing-user@example.com".to_string();
//
// let collaborators = client.domains().add_collaborator(1234, 1, email);
// ```
// # Arguments
// `account_id`: The account ID
// `domain_id`: The ID of the domain we want to list the collaborators of
// `email`: The email of the collaborator to be added
    pub fn add_collaborator(&self, account_id: u64, domain_id: u64, email: String) -> Result<DNSimpleResponse<Collaborator>, String> {
        let path = format!("/{}/domains/{}/collaborators", account_id, domain_id);

        let payload = AddCollaboratorPayload { email };

        self.client.post::<CollaboratorEndpoint>(&*path, serde_json::to_value(payload).unwrap())
    }

    // Removes a collaborator from a domain
//
// # Examples
//
// ```no_run
// use dnsimple_rust::dnsimple::{Client, new_client};
// use dnsimple_rust::dnsimple::domains::DomainCreationPayload;
//
// let client = new_client(true, String::from("AUTH_TOKEN"));
// let response = client.domains().remove_collaborator(1234, 42, 100);
// ```
//
// # Arguments
//
// `account_id`: The account ID
// `domain_id`: The ID of the domain we want to permanently delete
// `collaborator_id`: The id of the collaborator we want to remove from the domain
    pub fn remove_collaborator(&self, account_id: u64, domain_id: u64, collaborator_id: u64) -> DNSimpleEmptyResponse {
        let path = format!("/{}/domains/{}/collaborators/{}", account_id, domain_id, collaborator_id);
        self.client.delete(&*path)
    }
}
