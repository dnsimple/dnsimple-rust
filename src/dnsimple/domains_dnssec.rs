use std::collections::HashMap;
use crate::dnsimple::domains::Domains;
use crate::dnsimple::{DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, Filters, Paginate, Sort};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct DnssecStatus {
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

struct DnssecStatusEndpoint;

impl Endpoint for DnssecStatusEndpoint {
    type Output = DnssecStatus;
}

impl Domains<'_> {
    /// Enable DNSSEC for the domain in the account. This will sign the zone. If the domain is
    /// registered it will also add the DS record to the corresponding registry.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want to enable DNSSEC on
    pub fn enable_dnssec(&self, account_id: u64, domain: &str) -> Result<DNSimpleResponse<DnssecStatus>, String> {
        let path = format!("/{}/domains/{}/dnssec", account_id, domain);

        self.client.post::<DnssecStatusEndpoint>(&*path, Value::Null)
    }

    /// Disable DNSSEC for the domain in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want to disable DNSSEC on
    pub fn disable_dnssec(&self, account_id: u64, domain: &str) -> DNSimpleEmptyResponse {
        let path = format!("/{}/domains/{}/dnssec", account_id, domain);

        self.client.delete(&*path)
    }

    /// Get the status of DNSSEC, indicating whether it is currently enabled or disabled.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want retrieve the DNSSEC status from
    pub fn get_dnssec(&self, account_id: u64, domain: &str) -> Result<DNSimpleResponse<DnssecStatus>, String> {
        let path = format!("/{}/domains/{}/dnssec", account_id, domain);
        let filters = Filters::new(HashMap::new());
        let sort = Sort::new(String::from(""));
        let paginate = Paginate{ per_page: 0, page: 0 };

        self.client.get::<DnssecStatusEndpoint>(&*path, filters, sort, paginate)
    }
}