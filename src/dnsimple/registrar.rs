use crate::dnsimple::{Client, DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, Filters, Paginate, Sort};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::dnsimple::tlds::TldExtendedAttribute;

#[derive(Debug, Deserialize, Serialize)]
pub struct DomainCheck {
    pub domain: String,
    pub available: bool,
    pub premium: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DomainPremiumPrice {
    pub premium_price: String,
    pub action: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DomainPrices {
    pub domain: String,
    pub premium: bool,
    pub registration_price: f32,
    pub renewal_price: f32,
    pub transfer_price: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DomainRegistrationPayload {
    pub registrant_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_privacy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_attributes: Option<Vec<TldExtendedAttribute>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_price: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DomainRegistration {
    pub id: u64,
    pub domain_id: u64,
    pub registrant_id: u64,
    pub period: u64,
    pub state: String,
    pub auto_renew: bool,
    pub whois_privacy: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DomainTransferPayload {
    pub registrant_id: u64,
    pub auth_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_privacy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_attributes: Option<Vec<TldExtendedAttribute>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_price: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DomainTransfer {
    pub id: u64,
    pub domain_id: u64,
    pub registrant_id: u64,
    pub state: String,
    pub auto_renew: bool,
    pub whois_privacy: bool,
    pub status_description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DomainRenewalPayload {
    pub period: u64,
    pub premium_price: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DomainRenewal {
    pub id: u64,
    pub domain_id: u64,
    pub period: u64,
    pub state: String,
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
    type Output = DomainPrices;
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

pub struct Registrar<'a> {
    pub client: &'a Client
}

impl Registrar<'_> {
    /// Checks a domain name for availability.
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    pub fn check_domain(&self, account_id: u64, domain: String) -> Result<DNSimpleResponse<DomainCheck>, String> {
        let path = format!("/{}/registrar/domains/{}/check", account_id, domain);

        self.client.get::<DomainCheckEndpoint>(&*path, Filters{ filters: Default::default() }, Sort{ sort_by: "".to_string() }, Paginate{ per_page: 0, page: 0 })
    }

    /// Get the premium price for a domain.
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    #[deprecated(note="please use `get_domain_prices` instead")]
    pub fn check_domain_premium_price(&self, account_id: u64, domain: String, action: Option<String>) -> Result<DNSimpleResponse<DomainPremiumPrice>, String> {
        let path = format!("/{}/registrar/domains/{}/premium_price?action={}", account_id, domain, action.unwrap_or(String::from("registration")));

        self.client.get::<DomainPremiumPriceEndpoint>(&*path, Filters{ filters: Default::default() }, Sort{ sort_by: "".to_string() }, Paginate{ per_page: 0, page: 0 })
    }

    /// Get a domain’s price for registration, renewal, and transfer.
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    pub fn get_domain_prices(&self, account_id: u64, domain: String) -> Result<DNSimpleResponse<DomainPrices>, String> {
        let path = format!("/{}/registrar/domains/{}/prices", account_id, domain);

        self.client.get::<DomainPricesEndpoint>(&*path,Filters{ filters: Default::default() }, Sort{ sort_by: "".to_string() }, Paginate{ per_page: 0, page: 0 })
    }

    /// Get a domain’s price for registration, renewal, and transfer.
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    /// `payload`: The `DomainRegistrationPayload` with the information needed to register the domain
    pub fn register_domain(&self, account_id: u64, domain: String, payload: DomainRegistrationPayload) -> Result<DNSimpleResponse<DomainRegistration>, String> {
        let path = format!("/{}/registrar/domains/{}/registrations", account_id, domain);

        self.client.post::<DomainRegistrationEndpoint>(&*path, serde_json::to_value(payload).unwrap())
    }

    /// Transfer a domain name from another domain registrar into DNSimple.
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    /// `payload`: The `DomainTransferPayload` with the information needed to transfer the domain
    pub fn transfer_domain(&self, account_id: u64, domain: String, payload: DomainTransferPayload) -> Result<DNSimpleResponse<DomainTransfer>, String> {
        let path = format!("/{}/registrar/domains/{}/transfers", account_id, domain);

        self.client.post::<DomainTransferEndpoint>(&*path, serde_json::to_value(payload).unwrap())
    }

    /// Retrieves the details of an existing domain transfer.
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    /// `domain_transfer`: The domain transfer id
    pub fn get_domain_transfer(&self, account_id: u64, domain: String, domain_transfer: u64) -> Result<DNSimpleResponse<DomainTransfer>, String> {
        let path = format!("/{}/registrar/domains/{}/transfers/{}", account_id, domain, domain_transfer);

        self.client.get::<DomainTransferEndpoint>(&*path, Filters{ filters: Default::default() }, Sort{ sort_by: "".to_string() }, Paginate{ per_page: 0, page: 0 })
    }

    /// Cancels an in progress domain transfer.
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    /// `domain_transfer`: The domain transfer id
    pub fn cancel_domain_transfer(&self, account_id: u64, domain: String, domain_transfer: u64) -> Result<DNSimpleResponse<DomainTransfer>, String> {
        let path = format!("/{}/registrar/domains/{}/transfers/{}", account_id, domain, domain_transfer);

        self.client.delete_with_response::<DomainTransferEndpoint>(&*path)
    }

    /// Get a domain’s price for registration, renewal, and transfer.
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    /// `payload`: The `DomainRenewalPayload` with the information needed to renew the domain
    pub fn renew_domain(&self, account_id: u64, domain: String, payload: DomainRenewalPayload) -> Result<DNSimpleResponse<DomainRenewal>, String> {
        let path = format!("/{}/registrar/domains/{}/renewals", account_id, domain);

        self.client.post::<DomainRenewalEndpoint>(&*path, serde_json::to_value(payload).unwrap())
    }

    /// Authorize a domain transfer out
    ///
    /// # Attributes
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name
    pub fn transfer_domain_out(&self, account_id: u64, domain: String) -> DNSimpleEmptyResponse {
        let path = format!("/{}/registrar/domains/{}/authorize_transfer_out", account_id, domain);

        self.client.empty_post(&*path)
    }
}