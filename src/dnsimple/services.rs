use crate::dnsimple::{Client, DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, RequestOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Service {
    pub id: u64,
    pub name: String,
    pub sid: String,
    pub description: String,
    pub setup_description: Option<String>,
    pub requires_setup: bool,
    pub default_subdomain: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub settings: Vec<ServiceSetting>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceSetting {
    pub name: String,
    pub label: String,
    pub append: Option<String>,
    pub description: String,
    pub example: Option<String>,
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

pub struct Services<'a> {
    pub client: &'a Client
}

impl Services<'_> {
    /// List services
    pub fn list_services(&self, options: Option<RequestOptions>) -> Result<DNSimpleResponse<Vec<Service>>, String> {
        let path = "/services";

        self.client.get::<ServicesEndpoint>(&*path, options)
    }

    /// Retrieve a service
    ///
    /// # Arguments
    /// `service`: The service name or id
    pub fn get_service(&self, service: String) -> Result<DNSimpleResponse<Service>, String> {
        let path = format!("/services/{}", service);

        self.client.get::<ServiceEndpoint>(&path, None)
    }

    /// List services applied to a domain.
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `domain`: The domain name or id
    pub fn applied_services(&self, account_id: u64, domain: String, options: Option<RequestOptions>) -> Result<DNSimpleResponse<Vec<Service>>, String> {
        let path = format!("/{}/domains/{}/services", account_id, domain);

        self.client.get::<ServicesEndpoint>(&*path, options)
    }

    /// Applies a service to a domain.
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `domain`: The domain name or id
    /// `service`: The service name or id
    pub fn apply_service(&self, account_id: u64, domain: String, service: String) -> DNSimpleEmptyResponse {
        let path = format!("/{}/domains/{}/services/{}", account_id, domain, service);

        self.client.empty_post(&*path)
    }

    /// Unapplies a service to a domain.
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `domain`: The domain name or id
    /// `service`: The service name or id
    pub fn unapply_service(&self, account_id: u64, domain: String, service: String) -> DNSimpleEmptyResponse {
        let path = format!("/{}/domains/{}/services/{}", account_id, domain, service);

        self.client.delete(&*path)
    }
}