use std::collections::HashMap;

use crate::dnsimple::tlds::TldExtendedAttribute;
use crate::dnsimple::{Client, DNSimpleEmptyResponse, DNSimpleResponse, Endpoint};
use crate::errors::DNSimpleError;
use serde::{Deserialize, Serialize};

/// Represents the domain check
#[derive(Debug, Deserialize, Serialize)]
pub struct DomainCheck {
    /// The domain name that was checked.
    pub domain: String,
    /// Whether the domain name is available.
    pub available: bool,
    /// Whether the domain name is premium.
    pub premium: bool,
}

/// Represents a domain premium price
#[derive(Debug, Deserialize, Serialize)]
pub struct DomainPremiumPrice {
    /// The domain premium price
    pub premium_price: String,
    /// The action: registration/transfer/renewal
    pub action: String,
}

/// Represents the domain prices
#[derive(Debug, Deserialize, Serialize)]
pub struct DomainPrice {
    /// The domain name
    pub domain: String,
    /// Whether the domain is premium.
    pub premium: bool,
    /// The price for registration
    pub registration_price: f32,
    /// The price for renewal
    pub renewal_price: f32,
    /// The price for transfer
    pub transfer_price: f32,
}

/// The payload to register a domain
#[derive(Debug, Deserialize, Serialize)]
pub struct DomainRegistrationPayload {
    /// The associated registrant (contact) ID.
    pub registrant_id: u64,
    /// True if the domain WHOIS privacy was requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_privacy: Option<bool>,
    /// True if the domain auto-renew was requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// The extended attributes for the domain if needed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_attributes: Option<Vec<TldExtendedAttribute>>,
    /// The domain premium price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_price: Option<String>,
}

/// The domain registration
#[derive(Debug, Deserialize, Serialize)]
pub struct DomainRegistration {
    /// The domain registration ID in DNSimple.
    pub id: u64,
    ///  The associated domain ID.
    pub domain_id: u64,
    /// The associated registrant (contact) ID.
    pub registrant_id: u64,
    /// The number of years the domain was registered for.
    pub period: u64,
    /// The state of the renewal.
    pub state: String,
    /// True if the domain auto-renew was requested.
    pub auto_renew: bool,
    /// True if the domain WHOIS privacy was requested.
    pub whois_privacy: bool,
    /// When the domain renewal was created in DNSimple.
    pub created_at: String,
    /// When the domain renewal was last updated in DNSimple.
    pub updated_at: String,
}

/// Payload used to transfer a domain
#[derive(Debug, Deserialize, Serialize)]
pub struct DomainTransferPayload {
    /// The associated registrant (contact) ID.
    pub registrant_id: u64,
    /// The authorization code
    pub auth_code: String,
    /// True if the domain WHOIS privacy was requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_privacy: Option<bool>,
    /// True if the domain auto-renew was requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// The extended attributes for the domain if needed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_attributes: Option<Vec<TldExtendedAttribute>>,
    /// The domain premium price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_price: Option<String>,
}

/// Represents a domain transfer
#[derive(Debug, Deserialize, Serialize)]
pub struct DomainTransfer {
    /// The domain registration ID in DNSimple.
    pub id: u64,
    /// The associated domain ID.
    pub domain_id: u64,
    /// The associated registrant (contact) ID.
    pub registrant_id: u64,
    /// The state of the transfer.
    pub state: String,
    /// True if the domain auto-renew was requested.
    pub auto_renew: bool,
    /// True if the domain WHOIS privacy was requested.
    pub whois_privacy: bool,
    /// The reason if transfer failed.
    pub status_description: Option<String>,
    /// When the domain renewal was created in DNSimple.
    pub created_at: String,
    /// When the domain renewal was last updated in DNSimple.
    pub updated_at: String,
}

/// Payload to renew a domain
#[derive(Debug, Deserialize, Serialize)]
pub struct DomainRenewalPayload {
    /// The renewal period
    pub period: u64,
    /// The domain premium price
    pub premium_price: Option<String>,
}

/// Represents a domain renewal
#[derive(Debug, Deserialize, Serialize)]
pub struct DomainRenewal {
    /// The domain renewal ID in DNSimple.
    pub id: u64,
    /// The associated domain ID.
    pub domain_id: u64,
    /// The number of years the domain was renewed for.
    pub period: u64,
    /// The state of the renewal.
    pub state: String,
    ///  When the domain renewal was created in DNSimple.
    pub created_at: String,
    /// When the domain renewal was last updated in DNSimple.
    pub updated_at: String,
}

/// Payload to create a registrant change
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRegistrantChangePayload {
    pub domain_id: u64,
    pub contact_id: u64,
    pub extended_attributes: HashMap<String, String>,
}

/// Represents a registrant change
#[derive(Debug, Deserialize, Serialize)]
pub struct RegistrantChange {
    pub id: u64,
    #[serde(rename = "type")]
    pub typ: String,
    pub account_id: u64,
    pub contact_id: u64,
    pub domain_id: u64,
    pub state: String,
    pub extended_attributes: HashMap<String, String>,
    pub registry_owner_change: bool,
    pub irt_lock_lifted_by: String,
    pub created_at: String,
    pub updated_at: String,
}

struct DomainCheckEndpoint;

impl Endpoint for DomainCheckEndpoint {
    type Output = DomainCheck;
}

struct DomainPremiumPriceEndpoint;

impl Endpoint for DomainPremiumPriceEndpoint {
    type Output = DomainPremiumPrice;
}

struct DomainPricesEndpoint;

impl Endpoint for DomainPricesEndpoint {
    type Output = DomainPrice;
}

struct DomainRegistrationEndpoint;

impl Endpoint for DomainRegistrationEndpoint {
    type Output = DomainRegistration;
}

struct DomainTransferEndpoint;

impl Endpoint for DomainTransferEndpoint {
    type Output = DomainTransfer;
}

struct DomainRenewalEndpoint;

impl Endpoint for DomainRenewalEndpoint {
    type Output = DomainRenewal;
}

struct CreateRegistrantChangeEndpoint;

impl Endpoint for CreateRegistrantChangeEndpoint {
    type Output = RegistrantChange;
}

/// The Registrar Service handles the domains registrations of the DNSimple API.
///
/// See [API Documentation: registrar](https://developer.dnsimple.com/v2/registrar/)
pub struct Registrar<'a> {
    pub client: &'a Client,
}

impl Registrar<'_> {
    /// Checks a domain name for availability.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let domain_check = client.registrar().check_domain(1234, "example.com").unwrap().data.unwrap();
    /// ```
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    pub fn check_domain(
        &self,
        account_id: u64,
        domain: &str,
    ) -> Result<DNSimpleResponse<DomainCheck>, DNSimpleError> {
        let path = format!("/{}/registrar/domains/{}/check", account_id, domain);

        self.client.get::<DomainCheckEndpoint>(&path, None)
    }

    /// Get the premium price for a domain.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let domain_check = client.registrar().check_domain_premium_price(1234, "example.com", None).unwrap().data.unwrap();
    /// ```
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    #[deprecated(note = "please use `get_domain_prices` instead")]
    pub fn check_domain_premium_price(
        &self,
        account_id: u64,
        domain: &str,
        action: Option<String>,
    ) -> Result<DNSimpleResponse<DomainPremiumPrice>, DNSimpleError> {
        let path = format!(
            "/{}/registrar/domains/{}/premium_price?action={}",
            account_id,
            domain,
            action.unwrap_or_else(|| "registration".into())
        );

        self.client.get::<DomainPremiumPriceEndpoint>(&path, None)
    }

    /// Get a domain’s price for registration, renewal, and transfer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let domain_check = client.registrar().get_domain_prices(1234, "example.com").unwrap().data.unwrap();
    /// ```
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    pub fn get_domain_prices(
        &self,
        account_id: u64,
        domain: &str,
    ) -> Result<DNSimpleResponse<DomainPrice>, DNSimpleError> {
        let path = format!("/{}/registrar/domains/{}/prices", account_id, domain);

        self.client.get::<DomainPricesEndpoint>(&path, None)
    }

    /// Get the details of an existing domain registration.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let domain_check = client.registrar().get_domain_registration(1234, "example.com", 1556).unwrap().data.unwrap();
    /// ```
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    /// `domain_registration_id`: The domain registration id
    pub fn get_domain_registration(
        &self,
        account_id: u64,
        domain: &str,
        domain_registration_id: u64,
    ) -> Result<DNSimpleResponse<DomainRegistration>, DNSimpleError> {
        let path = format!(
            "/{}/registrar/domains/{}/registrations/{}",
            account_id, domain, domain_registration_id
        );

        self.client.get::<DomainRegistrationEndpoint>(&path, None)
    }

    /// Get the details of an existing domain transfer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let domain_check = client.registrar().get_domain_renewal(1234, "example.com", 1556).unwrap().data.unwrap();
    /// ```
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    /// `domain_renewal_id`: The domain renewal id
    pub fn get_domain_renewal(
        &self,
        account_id: u64,
        domain: &str,
        domain_renewal_id: u64,
    ) -> Result<DNSimpleResponse<DomainRenewal>, DNSimpleError> {
        let path = format!(
            "/{}/registrar/domains/{}/renewals/{}",
            account_id, domain, domain_renewal_id
        );

        self.client.get::<DomainRenewalEndpoint>(&path, None)
    }

    /// Get a domain’s price for registration, renewal, and transfer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    /// use dnsimple::dnsimple::registrar::DomainRegistrationPayload;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let payload = DomainRegistrationPayload {
    ///     registrant_id: 42,
    ///     whois_privacy: None,
    ///     auto_renew: None,
    ///     extended_attributes: None,
    ///     premium_price: None,
    /// };
    /// let domain_check = client.registrar().register_domain(1234, "example.com", payload).unwrap().data.unwrap();
    /// ```
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    /// `payload`: The `DomainRegistrationPayload` with the information needed to register the domain
    pub fn register_domain(
        &self,
        account_id: u64,
        domain: &str,
        payload: DomainRegistrationPayload,
    ) -> Result<DNSimpleResponse<DomainRegistration>, DNSimpleError> {
        let path = format!("/{}/registrar/domains/{}/registrations", account_id, domain);

        match serde_json::to_value(payload) {
            Ok(json) => self.client.post::<DomainRegistrationEndpoint>(&path, json),
            Err(_) => Err(DNSimpleError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }

    /// Transfer a domain name from another domain registrar into DNSimple.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    /// use dnsimple::dnsimple::registrar::DomainTransferPayload;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let payload = DomainTransferPayload {
    ///     registrant_id: 42,
    ///     auth_code: "Some code".to_string(),
    ///     whois_privacy: None,
    ///     auto_renew: None,
    ///     extended_attributes: None,
    ///     premium_price: None,
    /// };
    /// let domain_transfer = client.registrar().transfer_domain(1234, "example.com", payload).unwrap().data.unwrap();
    /// ```
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    /// `payload`: The `DomainTransferPayload` with the information needed to transfer the domain
    pub fn transfer_domain(
        &self,
        account_id: u64,
        domain: &str,
        payload: DomainTransferPayload,
    ) -> Result<DNSimpleResponse<DomainTransfer>, DNSimpleError> {
        let path = format!("/{}/registrar/domains/{}/transfers", account_id, domain);

        match serde_json::to_value(payload) {
            Ok(json) => self.client.post::<DomainTransferEndpoint>(&path, json),
            Err(_) => Err(DNSimpleError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }

    /// Retrieves the details of an existing domain transfer.
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    /// `domain_transfer`: The domain transfer id
    pub fn get_domain_transfer(
        &self,
        account_id: u64,
        domain: String,
        domain_transfer: u64,
    ) -> Result<DNSimpleResponse<DomainTransfer>, DNSimpleError> {
        let path = format!(
            "/{}/registrar/domains/{}/transfers/{}",
            account_id, domain, domain_transfer
        );

        self.client.get::<DomainTransferEndpoint>(&path, None)
    }

    /// Cancels an in progress domain transfer.
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    /// `domain_transfer`: The domain transfer id
    pub fn cancel_domain_transfer(
        &self,
        account_id: u64,
        domain: String,
        domain_transfer: u64,
    ) -> Result<DNSimpleResponse<DomainTransfer>, DNSimpleError> {
        let path = format!(
            "/{}/registrar/domains/{}/transfers/{}",
            account_id, domain, domain_transfer
        );

        self.client
            .delete_with_response::<DomainTransferEndpoint>(&path)
    }

    /// Get a domain’s price for registration, renewal, and transfer.
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    /// `payload`: The `DomainRenewalPayload` with the information needed to renew the domain
    pub fn renew_domain(
        &self,
        account_id: u64,
        domain: String,
        payload: DomainRenewalPayload,
    ) -> Result<DNSimpleResponse<DomainRenewal>, DNSimpleError> {
        let path = format!("/{}/registrar/domains/{}/renewals", account_id, domain);

        match serde_json::to_value(payload) {
            Ok(json) => self.client.post::<DomainRenewalEndpoint>(&path, json),
            Err(_) => Err(DNSimpleError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }

    /// Authorize a domain transfer out
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    pub fn transfer_domain_out(
        &self,
        account_id: u64,
        domain: String,
    ) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let path = format!(
            "/{}/registrar/domains/{}/authorize_transfer_out",
            account_id, domain
        );

        self.client.empty_post(&path)
    }

    pub fn create_registrant_change(
        &self,
        account_id: u64,
        payload: CreateRegistrantChangePayload,
    ) -> Result<DNSimpleResponse<RegistrantChange>, DNSimpleError> {
        let path = format!("/{}/registrar/registrant_changes", account_id);

        match serde_json::to_value(payload) {
            Ok(json) => self
                .client
                .post::<CreateRegistrantChangeEndpoint>(&path, json),
            Err(_) => Err(DNSimpleError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }
}
