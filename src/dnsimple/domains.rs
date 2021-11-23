use crate::dnsimple::{Client, DNSimpleEmptyResponse, DNSimpleResponse, Endpoint};
use serde::{Deserialize, Serialize};

/// Represents a domain
///
/// See [API Documentation: domains](https://developer.dnsimple.com/v2/domains/)
#[derive(Debug, Deserialize, Serialize)]
pub struct Domain {
    /// The domain ID in DNSimple
    pub id: u64,
    /// The account ID holding the domain
    pub account_id: u64,
    /// The registrants ID
    pub registrant_id: Option<u64>,
    /// The name of the domain
    pub name: String,
    /// The name of the domain in unicode
    pub unicode_name: String,
    /// The state of the domain
    pub state: String,
    /// Set to true if the domain will be auto-renewed
    pub auto_renew: bool,
    /// Set to true if the domain is WHOIS protected
    pub private_whois: bool,
    /// The day the domain will expire
    pub expires_on: Option<String>,
    /// The exact expiration time of the domain
    pub expires_at: Option<String>,
    /// When the domain was created
    pub created_at: String,
    /// When the domain was last updated
    pub updated_at: String
}

/// Represents the payload to be send when creating a domain
///
/// See [API Documentation: domains](https://developer.dnsimple.com/v2/domains/)
#[derive(Debug, Deserialize, Serialize)]
pub struct DomainCreationPayload {
    pub name: String,
}

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

struct ListDomainsEndpoint;

impl Endpoint for ListDomainsEndpoint {
    type Output = Vec<Domain>;
}

struct DomainEndpoint;

impl Endpoint for DomainEndpoint {
    type Output = Domain;
}

struct ListCollaboratorsEndpoint;

impl Endpoint for ListCollaboratorsEndpoint {
    type Output = Vec<Collaborator>;
}

struct CollaboratorEndpoint;

impl Endpoint for CollaboratorEndpoint {
    type Output = Collaborator;
}

/// The Domains Service handles the domains endpoint of the DNSimple API.
///
/// See [API Documentation: domains](https://developer.dnsimple.com/v2/domains/)
pub struct Domains<'a> {
    pub client: &'a Client
}

impl Domains<'_> {
    /// Lists the domains in the account
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple_rust::dnsimple::{Client, new_client};
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let domains_response = client.domains().list_domains(1234);
    /// ```
    ///
    /// # Arguments
    /// `account_id`: The account ID
    // pub fn list_domains(&self, account_id: u64) -> DNSimpleResponse<DomainsData> {
    pub fn list_domains(&self, account_id: u64) -> Result<DNSimpleResponse<Vec<Domain>>, String> {
        let path = format!("/{}/domains", account_id);
        self.client.get::<ListDomainsEndpoint>(&*path)
    }

    /// Adds a domain to the account.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple_rust::dnsimple::{Client, new_client};
    /// use dnsimple_rust::dnsimple::domains::DomainCreationPayload;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let domain_name = String::from("example-beta.com");
    ///
    /// let domains_response = client.domains().create_domain(1234, domain_name);
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `name`: The name of the domain we want to create
    // pub fn create_domain(&self, account_id: u64, name: String) -> DNSimpleResponse<DomainData> {
    pub fn create_domain(&self, account_id: u64, name: String) -> Result<DNSimpleResponse<Domain>, String> {
        let path = format!("/{}/domains", account_id);

        let payload = DomainCreationPayload { name };

        self.client.post::<DomainEndpoint>(&*path, serde_json::to_value(payload).unwrap())
    }

    /// Retrieves the details of an existing domain.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple_rust::dnsimple::{Client, new_client};
    /// use dnsimple_rust::dnsimple::domains::DomainCreationPayload;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let domains_response = client.domains().get_domain(1234, 42);
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain_id`: The ID of the domain we want to retrieve
    // pub fn get_domain(&self, account_id: u64, domain_id: u64) -> DNSimpleResponse<DomainData> {
    pub fn get_domain(&self, account_id: u64, domain_id: u64) -> Result<DNSimpleResponse<Domain>, String> {
        let path = format!("/{}/domains/{}", account_id, domain_id);

        self.client.get::<DomainEndpoint>(&*path)
    }

    /// Permanently deletes a domain from the account. It cannot be undone.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple_rust::dnsimple::{Client, new_client};
    /// use dnsimple_rust::dnsimple::domains::DomainCreationPayload;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let domains_response = client.domains().delete_domain(1234, 42);
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain_id`: The ID of the domain we want to permanently delete
    pub fn delete_domain(&self, account_id: u64, domain_id: u64) -> DNSimpleEmptyResponse {
        let path = format!("/{}/domains/{}", account_id, domain_id);
        self.client.delete(&*path)
    }

    /// List collaborators for the domain in the account.
    ///
    /// # Examples
    /// ```no_run
    /// use dnsimple_rust::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let collaborators = client.domains().list_collaborators(1234, 1);
    /// ```
    ///
    /// # Arguments
    /// `account_id`: The account ID
    /// `domain_id`: The ID of the domain we want to list the collaborators of
    pub fn list_collaborators(&self, account_id: u64, domain_id: u64) -> Result<DNSimpleResponse<Vec<Collaborator>>, String> {
        let path = format!("/{}/domains/{}/collaborators", account_id, domain_id);
        self.client.get::<ListCollaboratorsEndpoint>(&*path)
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

        // TODO: Remove the unwrap and re-test
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