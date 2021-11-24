use crate::dnsimple::domains::Domains;
use crate::dnsimple::{DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, Filters, Paginate, Sort};
use serde::{Deserialize, Serialize};

struct EmailForwardsListEndpoint;

impl Endpoint for EmailForwardsListEndpoint {
    type Output = Vec<EmailForwardsInList>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailForwardsInList {
    pub id: u64,
    pub domain_id: u64,
    pub from: String,
    pub to: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailForward {
    pub id: u64,
    pub domain_id: u64,
    pub alias_email: String,
    pub destination_email: String,
    pub created_at: String,
    pub updated_at: String,
    pub from: String,
    pub to: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailForwardPayload {
    pub alias_name: String,
    pub destination_email: String,
}

struct EmailForwardEndpoint;

impl Endpoint for EmailForwardEndpoint {
    type Output = EmailForward;
}

impl Domains<'_> {

    /// List email forwards for the domain in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want list the email forwards
    pub fn list_email_forwards(&self, account_id: u64, domain: String, filters: Filters, sort: Sort, paginate: Paginate) -> Result<DNSimpleResponse<Vec<EmailForwardsInList>>, String> {
        let path = format!("/{}/domains/{}/email_forwards", account_id, domain);

        self.client.get::<EmailForwardsListEndpoint>(&*path, filters, sort, paginate)
    }

    /// Create an email forward
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want list the email forwards
    /// `payload`: The `EmailForwardPayload` with the data needed to create the email forward
    pub fn create_email_forward(&self, account_id: u64, domain: String, payload: EmailForwardPayload) -> Result<DNSimpleResponse<EmailForward>, String> {
        let path = format!("/{}/domains/{}/email_forwards", account_id, domain);

        self.client.post::<EmailForwardEndpoint>(&*path, serde_json::to_value(payload).unwrap())
    }

    /// Retrieve an email forward
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want list the email forwards
    /// `email_forward`: The email forward id
    pub fn get_email_forward(&self, account_id: u64, domain: String, email_forward: u64) -> Result<DNSimpleResponse<EmailForward>, String> {
        let path = format!("/{}/domains/{}/email_forwards/{}", account_id, domain, email_forward);

        self.client.get::<EmailForwardEndpoint>(&*path, Filters { filters: Default::default() }, Sort { sort_by: "".to_string() }, Paginate { per_page: 0, page: 0 })
    }

    /// Delete the email forward from the domain.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want list the email forwards
    /// `email_forward`: The email forward id
    pub fn delete_email_forward(&self, account_id: u64, domain: String, email_forward: i32) -> DNSimpleEmptyResponse {
        let path = format!("/{}/domains/{}/email_forwards/{}", account_id, domain, email_forward);

        self.client.delete(&*path)
    }
 }