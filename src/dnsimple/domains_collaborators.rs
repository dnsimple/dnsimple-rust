use crate::dnsimple::domains::Domains;
use crate::dnsimple::{DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, RequestOptions};
use crate::errors::DNSimpleError;
use serde::{Deserialize, Serialize};

/// Represents a collaborator
#[derive(Debug, Deserialize, Serialize)]
pub struct Collaborator {
    /// The collaborator ID in DNSimple.
    pub id: u64,
    /// The associated domain ID.
    pub domain_id: u64,
    /// The associated domain name.
    pub domain_name: String,
    /// The user ID, if the collaborator accepted the invitation.
    pub user_id: Option<u64>,
    /// The user email.
    pub user_email: String,
    /// Invitation
    pub invitation: bool,
    ///  When the collaborator was created in DNSimple.
    pub created_at: String,
    /// When the collaborator was last updated in DNSimple.
    pub updated_at: String,
    /// When the collaborator has accepted the invitation.
    pub accepted_at: Option<String>,
}

/// The payload used to add a collaborator
#[derive(Debug, Deserialize, Serialize)]
pub struct AddCollaboratorPayload {
    /// The email of the collaborator
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

/// The domains collaborators set of endpoints
///
/// See [API Documentation: domains/collaborators](https://developer.dnsimple.com/v2/domains/collaborators)
impl Domains<'_> {
    /// List collaborators for the domain in the account.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::{Filters, new_client, Paginate, Sort};
    /// use std::collections::HashMap;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let collaborators = client.domains().list_collaborators(1234, 1, None).unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain_id`: The ID of the domain we want to list the collaborators from
    /// `options`: The `RequestOptions`
    ///            - Pagination
    #[deprecated(
        note = "`DomainCollaborators` have been deprecated and will be removed in the next major version. Please use our Domain Access Control feature."
    )]
    pub fn list_collaborators(
        &self,
        account_id: u64,
        domain_id: u64,
        options: Option<RequestOptions>,
    ) -> Result<DNSimpleResponse<Vec<Collaborator>>, DNSimpleError> {
        let path = format!("/{}/domains/{}/collaborators", account_id, domain_id);

        self.client.get::<ListCollaboratorsEndpoint>(&path, options)
    }

    /// At the time of the add, a collaborator may or may not have a DNSimple account.
    ///
    /// In case the collaborator doesn't have a DNSimple account, the system will invite them to
    /// register to DNSimple first and then to accept the collaboration invitation.
    ///
    /// In the other case, they are automatically added to the domain as collaborator. They can
    /// decide to reject the invitation later.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let email = "existing-user@example.com";
    ///
    /// let collaborator = client.domains().add_collaborator(1234, 1, email).unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain_id`: The ID of the domain we want to list the collaborators of
    /// `email`: The email of the collaborator to be added
    #[deprecated(
        note = "`DomainCollaborators` have been deprecated and will be removed in the next major version. Please use our Domain Access Control feature."
    )]
    pub fn add_collaborator(
        &self,
        account_id: u64,
        domain_id: u64,
        email: &str,
    ) -> Result<DNSimpleResponse<Collaborator>, DNSimpleError> {
        let path = format!("/{}/domains/{}/collaborators", account_id, domain_id);

        let payload = AddCollaboratorPayload {
            email: email.into(),
        };
        match serde_json::to_value(payload) {
            Ok(json) => self.client.post::<CollaboratorEndpoint>(&path, json),
            Err(_) => Err(DNSimpleError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }

    /// Removes a collaborator from a domain
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::{Client, new_client};
    /// use dnsimple::dnsimple::domains::DomainCreationPayload;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let response = client.domains().remove_collaborator(1234, 42, 100);
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain_id`: The ID of the domain we want to permanently delete
    /// `collaborator_id`: The id of the collaborator we want to remove from the domain
    #[deprecated(
        note = "`DomainCollaborators` have been deprecated and will be removed in the next major version. Please use our Domain Access Control feature."
    )]
    pub fn remove_collaborator(
        &self,
        account_id: u64,
        domain_id: u64,
        collaborator_id: u64,
    ) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let path = format!(
            "/{}/domains/{}/collaborators/{}",
            account_id, domain_id, collaborator_id
        );
        self.client.delete(&path)
    }
}
