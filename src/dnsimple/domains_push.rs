use crate::dnsimple::domains::Domains;
use crate::dnsimple::{DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, Filters, Paginate, Sort};
use serde::{Deserialize, Serialize};

struct DomainPushesListEndpoint;

impl Endpoint for DomainPushesListEndpoint {
    type Output = Vec<DomainPush>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DomainPush {
    pub id: u64,
    pub domain_id: u64,
    pub contact_id: Option<u64>,
    pub account_id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub accepted_at: Option<String>
}

impl Domains<'_> {

    /// List pending pushes for the target account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account id
    pub fn list_pushes(&self, account_id: u64, paginate: Paginate) -> Result<DNSimpleResponse<Vec<DomainPush>>, String> {
        let path = format!("/{}/domains/pushes", account_id);

        self.client.get::<DomainPushesListEndpoint>(&*path, Filters { filters: Default::default() }, Sort { sort_by: "".to_string() }, paginate)
    }

    /// Accept a push
    ///
    /// # Arguments
    ///
    /// `account_id`: The account id
    /// `push_id`: The push id
    pub fn accept_push(&self, account_id: u64, push_id: u64) -> DNSimpleEmptyResponse {
        let path = format!("/{}/domains/pushes/{}", account_id, push_id);

        self.client.empty_post(&*path)
    }

    /// Reject a push
    ///
    /// # Arguments
    ///
    /// `account_id`: The account id
    /// `push_id`: The push id
    pub fn reject_push(&self, account_id: u64, push_id: u64) -> DNSimpleEmptyResponse {
        let path = format!("/{}/domains/pushes/{}", account_id, push_id);

        self.client.delete(&*path)
    }
}
