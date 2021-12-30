use crate::dnsimple::registrar::Registrar;
use crate::dnsimple::{DNSimpleResponse, Endpoint};
use crate::errors::DNSimpleError;
use serde::Deserialize;
use serde_json::Value;

/// Represents the whois privacy data
#[derive(Debug, Deserialize)]
pub struct WhoisPrivacy {
    /// The whois privacy id in DNSimple
    pub id: u64,
    /// The associated domain ID.
    pub domain_id: u64,
    /// The date the whois privacy will expire on.
    pub expires_on: Option<String>,
    /// Whether the whois privacy is enabled for the domain.
    pub enabled: Option<bool>,
    /// When the whois privacy was created in DNSimple.
    pub created_at: String,
    /// When the whois privacy was created in DNSimple.
    pub updated_at: String,
}

/// Represents the whois privacy renewal data
#[derive(Debug, Deserialize)]
pub struct WhoisPrivacyRenewal {
    /// The renewal id in DNSimple
    pub id: u64,
    /// The associated domain ID.
    pub domain_id: u64,
    /// The associated WHOIS Privacy ID.
    pub whois_privacy_id: u64,
    /// The WHOIS Privacy order state.
    pub state: String,
    /// The date the WHOIS Privacy will expire on.
    pub expires_on: String,
    /// Whether the WHOIS Privacy is enabled for the domain.
    pub enabled: bool,
    /// When the WHOIS Privacy was created in DNSimple.
    pub created_at: String,
    /// When the WHOIS Privacy was last updated in DNSimple.
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
    pub fn get_whois_privacy(
        &self,
        account_id: u64,
        domain: String,
    ) -> Result<DNSimpleResponse<WhoisPrivacy>, DNSimpleError> {
        let path = format!("/{}/registrar/domains/{}/whois_privacy", account_id, domain);

        self.client.get::<WhoisPrivacyEndpoint>(&*path, None)
    }

    /// Enable WHOIS privacy
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The domain name or id
    pub fn enable_whois_privacy(
        &self,
        account_id: u64,
        domain: String,
    ) -> Result<DNSimpleResponse<WhoisPrivacy>, DNSimpleError> {
        let path = format!("/{}/registrar/domains/{}/whois_privacy", account_id, domain);

        self.client.put::<WhoisPrivacyEndpoint>(&*path, Value::Null)
    }

    /// Enable WHOIS privacy
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The domain name or id
    pub fn disable_whois_privacy(
        &self,
        account_id: u64,
        domain: String,
    ) -> Result<DNSimpleResponse<WhoisPrivacy>, DNSimpleError> {
        let path = format!("/{}/registrar/domains/{}/whois_privacy", account_id, domain);

        self.client
            .delete_with_response::<WhoisPrivacyEndpoint>(&*path)
    }

    /// Renew WHOIS privacy
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The domain name or id
    pub fn renew_whois_privacy(
        &self,
        account_id: u64,
        domain: String,
    ) -> Result<DNSimpleResponse<WhoisPrivacyRenewal>, DNSimpleError> {
        let path = format!("/{}/registrar/domains/{}/whois_privacy", account_id, domain);

        self.client
            .post::<WhoisPrivacyRenewalEndpoint>(&*path, Value::Null)
    }
}
