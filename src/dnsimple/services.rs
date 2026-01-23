use crate::dnsimple::{Client, DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, RequestOptions};
use crate::errors::DNSimpleError;
use serde::{Deserialize, Serialize};

/// Represents a service in DNSimple
#[derive(Debug, Deserialize, Serialize)]
pub struct Service {
    /// The service ID in DNSimple.
    pub id: u64,
    /// The service name.
    pub name: String,
    ///  A string ID for the service.
    pub sid: String,
    /// The service description.
    pub description: String,
    /// The service setup description.
    pub setup_description: Option<String>,
    /// Whether the service requires extra setup.
    pub requires_setup: bool,
    /// The default subdomain where the service will be applied.
    pub default_subdomain: Option<String>,
    /// When the service was created in DNSimple
    pub created_at: String,
    /// When the service was last updated in DNSimple
    pub updated_at: String,
    /// The array of settings to setup this service, if setup is required.
    pub settings: Vec<ServiceSetting>,
}

/// Represents a service setting
#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceSetting {
    /// The setting name.
    pub name: String,
    /// The setting label.
    pub label: String,
    /// A suffix to be appended to the setting value.
    pub append: Option<String>,
    /// The setting description.
    pub description: String,
    /// The setting description.
    pub example: Option<String>,
    /// Whether the setting requires a password.
    pub password: bool,
}

struct ServicesEndpoint;

impl Endpoint for ServicesEndpoint {
    type Output = Vec<Service>;
}

struct ServiceEndpoint;

impl Endpoint for ServiceEndpoint {
    type Output = Service;
}

/// The Services Service handles the domains services of the DNSimple API.
///
/// See [API Documentation: services](https://developer.dnsimple.com/v2/services/)
pub struct Services<'a> {
    pub client: &'a Client,
}

impl Services<'_> {
    /// List services
    pub async fn list_services(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<DNSimpleResponse<Vec<Service>>, DNSimpleError> {
        let path = "/services";

        self.client.get::<ServicesEndpoint>(path, options).await
    }

    /// Retrieve a service
    ///
    /// # Arguments
    /// `service`: The service name or id
    pub async fn get_service(&self, service: String) -> Result<DNSimpleResponse<Service>, DNSimpleError> {
        let path = format!("/services/{}", service);

        self.client.get::<ServiceEndpoint>(&path, None).await
    }

    /// List services applied to a domain.
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `domain`: The domain name or id
    pub async fn applied_services(
        &self,
        account_id: u64,
        domain: String,
        options: Option<RequestOptions>,
    ) -> Result<DNSimpleResponse<Vec<Service>>, DNSimpleError> {
        let path = format!("/{}/domains/{}/services", account_id, domain);

        self.client.get::<ServicesEndpoint>(&path, options).await
    }

    /// Applies a service to a domain.
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `domain`: The domain name or id
    /// `service`: The service name or id
    pub async fn apply_service(
        &self,
        account_id: u64,
        domain: String,
        service: String,
    ) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let path = format!("/{}/domains/{}/services/{}", account_id, domain, service);

        self.client.empty_post(&path).await
    }

    /// Unapplies a service to a domain.
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `domain`: The domain name or id
    /// `service`: The service name or id
    pub async fn unapply_service(
        &self,
        account_id: u64,
        domain: String,
        service: String,
    ) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let path = format!("/{}/domains/{}/services/{}", account_id, domain, service);

        self.client.delete(&path).await
    }
}
