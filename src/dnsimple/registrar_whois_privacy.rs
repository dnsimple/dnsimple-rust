use crate::dnsimple::{DNSimpleResponse, Endpoint};
use crate::dnsimple::registrar::Registrar;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct WhoisPrivacy {
    pub id: u64,
    pub domain_id: u64,
    pub expires_on: Option<String>,
    pub enabled: Option<bool>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct WhoisPrivacyRenewal {
    pub id: u64,
    pub domain_id: u64,
    pub whois_privacy_id: u64,
    pub state: String,
    pub expires_on: String,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

struct WhoisPrivacyEndpoint;

impl Endpoint for WhoisPrivacyEndpoint {
    type Output = WhoisPrivacy;
}

struct WhoisPrivacyRenewalEndpoint;

impl Endpoint for WhoisPrivacyRenewalEndpoint {
    type Output = WhoisPrivacyRenewal;
}

impl Registrar<'_> {

    /// Retrieve the domain WHOIS privacy
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The domain name or id
    pub fn get_whois_privacy(&self, account_id: u64, domain: String) -> Result<DNSimpleResponse<WhoisPrivacy>, String> {
        let path = format!("/{}/registrar/domains/{}/whois_privacy", account_id, domain);

        self.client.get::<WhoisPrivacyEndpoint>(&*path, None)
    }

    /// Enable WHOIS privacy
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The domain name or id
    pub fn enable_whois_privacy(&self, account_id: u64, domain: String) -> Result<DNSimpleResponse<WhoisPrivacy>, String> {
        let path = format!("/{}/registrar/domains/{}/whois_privacy", account_id, domain);

        self.client.put::<WhoisPrivacyEndpoint>(&*path, Value::Null)
    }

    /// Enable WHOIS privacy
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The domain name or id
    pub fn disable_whois_privacy(&self, account_id: u64, domain: String) -> Result<DNSimpleResponse<WhoisPrivacy>, String> {
        let path = format!("/{}/registrar/domains/{}/whois_privacy", account_id, domain);

        self.client.delete_with_response::<WhoisPrivacyEndpoint>(&*path)
    }

    /// Renew WHOIS privacy
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The domain name or id
    pub fn renew_whois_privacy(&self, account_id: u64, domain: String) -> Result<DNSimpleResponse<WhoisPrivacyRenewal>, String> {
        let path = format!("/{}/registrar/domains/{}/whois_privacy", account_id, domain);

        self.client.post::<WhoisPrivacyRenewalEndpoint>(&*path, Value::Null)
    }
}