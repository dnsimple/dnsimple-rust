use crate::dnsimple::{Client, DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, Filters, Paginate, RequestOptions, Sort};
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

struct ListDomainsEndpoint;

impl Endpoint for ListDomainsEndpoint {
    type Output = Vec<Domain>;
}

struct DomainEndpoint;

impl Endpoint for DomainEndpoint {
    type Output = Domain;
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
    /// use std::collections::HashMap;
    /// use dnsimple_rust::dnsimple::{Client, new_client, Filters, Sort, Paginate};
    /// let filters = Filters::new(HashMap::new());
    /// let sort = Sort::new(String::from(""));
    /// let paginate = Paginate{ per_page: 0, page: 0 };
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let domains_response = client.domains().list_domains(1234, filters, sort, paginate);
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    // pub fn list_domains(&self, account_id: u64) -> DNSimpleResponse<DomainsData> {
    pub fn list_domains(&self, account_id: u64, filters: Filters, sort: Sort, paginate: Paginate) -> Result<DNSimpleResponse<Vec<Domain>>, String> {
        let path = format!("/{}/domains", account_id);
        self.client.get::<ListDomainsEndpoint>(&*path, Option::from(RequestOptions { filters: Some(filters), sort: Some(sort), paginate: Some(paginate) }))
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
        self.client.get::<DomainEndpoint>(&*path, None)
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
}