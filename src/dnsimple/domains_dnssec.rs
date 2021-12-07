use crate::dnsimple::domains::Domains;
use crate::dnsimple::{DNSimpleEmptyResponse, DNSimpleResponse, Endpoint};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents DNSSEC
#[derive(Debug, Deserialize, Serialize)]
pub struct Dnssec {
    /// True if DNSSEC is enabled on the domain, otherwise false
    pub enabled: bool,
    /// When DNSSEC was enabled (or disabled)
    pub created_at: String,
    /// When DNSSEC was last updated
    pub updated_at: String,
}

struct DnssecStatusEndpoint;

impl Endpoint for DnssecStatusEndpoint {
    type Output = Dnssec;
}

/// The domains dnssec set of endpoints
///
/// See [API Documentation: domains/dnssec](https://developer.dnsimple.com/v2/domains/dnssec)
impl Domains<'_> {
    /// Enable DNSSEC for the domain in the account. This will sign the zone. If the domain is
    /// registered it will also add the DS record to the corresponding registry.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let dnssec = client.domains().enable_dnssec(1234, "example.com").unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want to enable DNSSEC on
    pub fn enable_dnssec(&self, account_id: u64, domain: &str) -> Result<DNSimpleResponse<Dnssec>, String> {
        let path = format!("/{}/domains/{}/dnssec", account_id, domain);

        self.client.post::<DnssecStatusEndpoint>(&*path, Value::Null)
    }

    /// Disable DNSSEC for the domain in the account.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// client.domains().disable_dnssec(1234, "example.com");
    /// ```
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
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let dnssec = client.domains().get_dnssec(1234, "example.com").unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want retrieve the DNSSEC status from
    pub fn get_dnssec(&self, account_id: u64, domain: &str) -> Result<DNSimpleResponse<Dnssec>, String> {
        let path = format!("/{}/domains/{}/dnssec", account_id, domain);

        self.client.get::<DnssecStatusEndpoint>(&*path, None)
    }
}