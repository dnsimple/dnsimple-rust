use crate::dnsimple::accounts::Accounts;
use crate::dnsimple::certificates::Certificates;
use crate::dnsimple::contacts::Contacts;
use crate::dnsimple::domains::Domains;
use crate::dnsimple::identity::Identity;
use crate::dnsimple::oauth::OAuth;
use crate::dnsimple::registrar::Registrar;
use crate::dnsimple::services::Services;
use crate::dnsimple::templates::Templates;
use crate::dnsimple::tlds::Tlds;
use crate::dnsimple::vanity_name_servers::VanityNameServers;
use crate::dnsimple::webhooks::Webhooks;
use crate::dnsimple::zones::Zones;
use crate::errors::DNSimpleError;
use serde;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

pub mod accounts;
pub mod certificates;
pub mod contacts;
pub mod domains;
pub mod domains_dnssec;
pub mod domains_email_forwards;
pub mod domains_push;
pub mod domains_signer_records;
pub mod identity;
pub mod oauth;
pub mod registrar;
pub mod registrar_auto_renewal;
pub mod registrar_name_servers;
pub mod registrar_registrant_changes;
pub mod registrar_transfer_lock;
pub mod registrar_whois_privacy;
pub mod services;
pub mod templates;
pub mod tlds;
pub mod vanity_name_servers;
pub mod webhooks;
pub mod zones;
pub mod zones_records;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_USER_AGENT: &str = "dnsimple-rust/";

const API_VERSION: &str = "v2";
const DEFAULT_BASE_URL: &str = "https://api.dnsimple.com";
const DEFAULT_SANDBOX_URL: &str = "https://api.sandbox.dnsimple.com";

/// Represents the Rust client for the DNSimple API V2
///
/// The client is your entrypoint to the DNSimple API. Using it
/// you will be able to call all the endpoints of the DNSimple API
/// and their respective functions.
///
/// # Examples
///
/// ```no_run
/// use dnsimple::dnsimple::{Client, new_client};
///
/// #[tokio::main]
/// async fn main() {
///     let client = new_client(true, String::from("AUTH_TOKEN"));
///     let identity = client.identity().whoami().await.unwrap().data.unwrap();
///
///     let account = identity.account.unwrap();
/// }
/// ```
///
pub struct Client {
    base_url: String,
    user_agent: String,
    auth_token: String,
    pub _client: reqwest::Client,
}

/// Defines the Endpoint trait for the different API endpoints
pub trait Endpoint {
    type Output: DeserializeOwned;
}

/// Represents the response from an API call
#[derive(Debug)]
pub struct DNSimpleResponse<T> {
    /// The maximum number of requests you can perform per hour.
    pub rate_limit: String,
    /// The number of requests remaining in the current rate limit window.
    pub rate_limit_remaining: String,
    /// The time at which the current rate limit window in [Unix time](https://en.wikipedia.org/wiki/Unix_time) format.
    pub rate_limit_reset: String,
    /// The HTTP Status Code
    pub status: u16,
    /// The object or a `Vec<T>` of objects (the type `T` will depend on the endpoint).
    pub data: Option<T>,
    /// Any API endpoint that returns a list of items requires pagination.
    pub pagination: Option<Pagination>,
    /// The body as a JSON `Value`
    pub body: Option<Value>,
}

/// Any API endpoint that returns a list of items requires pagination.
/// By default we will return 30 records from any listing endpoint. If an API endpoint returns
/// a list of items, then it will include a pagination object that contains pagination
/// information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    /// The page currently returned (default: 1)
    pub current_page: u64,
    /// The number of entries returned per page (default: 30)
    pub per_page: u64,
    /// The total number of entries available in the entire collection
    pub total_entries: u64,
    /// The total number of pages available given the current `per_page` value
    pub total_pages: u64,
}

/// When you can send some options into the request (i.e. for pagination).
pub struct RequestOptions {
    /// Filtering makes it possible to ask only for the exact subset of data that you you're looking for.
    pub filters: Option<Filters>,
    /// API v2 results are implicitly sorted according to policies that vary from endpoint to endpoint.
    pub sort: Option<Sort>,
    /// Pagination options
    pub paginate: Option<Paginate>,
}

/// Represents an empty response from the DNSimple API
/// (_these type of responses happen when issuing DELETE commands for example_)
pub struct DNSimpleEmptyResponse {
    /// The maximum number of requests you can perform per hour.
    pub rate_limit: String,
    /// The number of requests remaining in the current rate limit window.
    pub rate_limit_remaining: String,
    /// The time at which the current rate limit window in [Unix time](https://en.wikipedia.org/wiki/Unix_time) format.
    pub rate_limit_reset: String,
    /// The HTTP Status Code
    pub status: u16,
}

/// Filtering makes it possible to ask only for the exact subset of data that you you're looking for.
//
// With potential hundreds of result entries, it's convenient to apply a filter and receive only the
// interesting data.
#[derive(Debug)]
pub struct Filters {
    pub filters: HashMap<String, String>,
}

impl Filters {
    pub fn new(filters: HashMap<String, String>) -> Filters {
        Filters { filters }
    }
}

/// API v2 results are implicitly sorted according to policies that vary from endpoint to endpoint.
//
// You can decide your own sorting policy for each single API call via the sort parameter.
//
// This parameter accepts a set of comma separated key-value pairs: the name of a field and the
// order criteria (asc for ascending and desc for descending).
//
// The order of fields is relevant, as it will determine the priority of the sorting policies.
#[derive(Debug)]
pub struct Sort {
    pub sort_by: String,
}

impl Sort {
    pub fn new(sort_by: String) -> Sort {
        Sort { sort_by }
    }
}

/// The pagination instructions for the request
pub struct Paginate {
    /// The number of items you want
    pub per_page: u32,
    /// The page number
    pub page: u32,
}

/// Helper function to create a new client
///
/// Make sure you use this to create your client.
///
/// # Examples
///
/// ```no_run
/// use dnsimple::dnsimple::{Client, new_client};
///
/// let client = new_client(true, String::from("AUTH_TOKEN"));
/// ```
///
/// # Arguments
///
/// `sandbox`: `true` if you want to run in the sandbox environment, otherwise `false`
/// `token`: the bearer authentication token
pub fn new_client(sandbox: bool, token: String) -> Client {
    let mut url = DEFAULT_BASE_URL;
    if sandbox {
        url = DEFAULT_SANDBOX_URL;
    }

    Client {
        base_url: String::from(url),
        user_agent: DEFAULT_USER_AGENT.to_owned() + VERSION,
        auth_token: token,
        _client: reqwest::Client::new(),
    }
}

impl Client {
    ///Returns the `accounts` service attached to this client
    pub fn accounts(&self) -> Accounts<'_> {
        Accounts { client: self }
    }

    /// Returns the `contacts` service attached to this client
    pub fn contacts(&self) -> Contacts<'_> {
        Contacts { client: self }
    }

    /// Returns the `certificates` service attached to this client
    pub fn certificates(&self) -> Certificates<'_> {
        Certificates { client: self }
    }

    /// Returns the `domains` service attached to this client
    pub fn domains(&self) -> Domains<'_> {
        Domains { client: self }
    }

    /// Returns the `identity` service attached to this client
    pub fn identity(&self) -> Identity<'_> {
        Identity { client: self }
    }

    /// Returns the `oauth` service attached to this client
    pub fn oauth(&self) -> OAuth<'_> {
        OAuth { client: self }
    }

    /// Returns the `registrar` service attached to this client
    pub fn registrar(&self) -> Registrar<'_> {
        Registrar { client: self }
    }

    /// Returns the `services` service attached to this client
    pub fn services(&self) -> Services<'_> {
        Services { client: self }
    }

    /// Returns the `templates` service attached to this client
    pub fn templates(&self) -> Templates<'_> {
        Templates { client: self }
    }

    /// Returns the `tlds` service attached to this endpoint
    pub fn tlds(&self) -> Tlds<'_> {
        Tlds { client: self }
    }

    /// Returns the `vanity_name_servers` service attached to this endpoint
    pub fn vanity_name_servers(&self) -> VanityNameServers<'_> {
        VanityNameServers { client: self }
    }

    /// Returns the `webhooks` service attached to this endpoint
    pub fn webhooks(&self) -> Webhooks<'_> {
        Webhooks { client: self }
    }

    /// Returns the `zones` service attached to this endpoint
    pub fn zones(&self) -> Zones<'_> {
        Zones { client: self }
    }

    /// Convenience function to change the base url in runtime (used internally for
    /// testing).
    ///
    /// Note that if you want to do this you will have to declare your client mutable.
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::{Client, new_client};
    /// let mut client = new_client(true, String::from("ACCESS_TOKEN"));
    /// client.set_base_url("https://example.com");
    /// ```
    ///
    /// # Arguments
    ///
    /// `url`: The url we want to change the base url to.
    pub fn set_base_url(&mut self, url: &str) {
        self.base_url = String::from(url);
    }

    /// Returns the current url (including the `API_VERSION` as part of the path).
    pub fn versioned_url(&self) -> String {
        let mut url = String::from(&self.base_url);
        url.push('/');
        url.push_str(API_VERSION);
        url
    }

    /// Sends a GET request to the DNSimple API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `options`: optionally a `RequestOptions` with things like pagination, filtering and sorting
    pub async fn get<E: Endpoint>(
        &self,
        path: &str,
        options: Option<RequestOptions>,
    ) -> Result<DNSimpleResponse<E::Output>, DNSimpleError> {
        let request = self.build_get_request(path, options);
        self.call::<E>(request).await
    }

    /// Sends a POST request to the DNSimple API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `data`: the json payload to be sent to the server
    pub async fn post<E: Endpoint>(
        &self,
        path: &str,
        data: Value,
    ) -> Result<DNSimpleResponse<<E as Endpoint>::Output>, DNSimpleError> {
        let request = self.build_post_request(path);
        self.call_with_payload::<E>(request, data).await
    }

    /// Sends a POST request to the DNSimple API without any payload
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub async fn empty_post(&self, path: &str) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let request = self.build_post_request(path);
        self.call_empty(request).await
    }

    /// Sends a PUT request to the DNSimple API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `data`: the json payload to be sent to the server
    pub async fn put<E: Endpoint>(
        &self,
        path: &str,
        data: Value,
    ) -> Result<DNSimpleResponse<<E as Endpoint>::Output>, DNSimpleError> {
        let request = self.build_put_request(path);
        self.call_with_payload::<E>(request, data).await
    }

    /// Sends a PUT request to the DNSimple API without any payload
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub async fn empty_put(&self, path: &str) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let request = self.build_put_request(path);
        self.call_empty(request).await
    }

    /// Sends a PATCH request to the DNSimple API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `data`: the json payload to be sent to the server
    pub async fn patch<E: Endpoint>(
        &self,
        path: &str,
        data: Value,
    ) -> Result<DNSimpleResponse<<E as Endpoint>::Output>, DNSimpleError> {
        let request = self.build_patch_request(path);
        self.call_with_payload::<E>(request, data).await
    }

    /// Sends a DELETE request to the DNSimple API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub async fn delete(&self, path: &str) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let request = self.build_delete_request(path);
        self.call_empty(request).await
    }

    /// Sends a DELETE request to the DNSimple API returning a response containing a `DNSimpleResponse`
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub async fn delete_with_response<E: Endpoint>(
        &self,
        path: &str,
    ) -> Result<DNSimpleResponse<E::Output>, DNSimpleError> {
        let request = self.build_delete_request(path);
        self.call::<E>(request).await
    }

    async fn call_with_payload<E: Endpoint>(
        &self,
        request: reqwest::RequestBuilder,
        data: Value,
    ) -> Result<DNSimpleResponse<E::Output>, DNSimpleError> {
        let response = request
            .json(&data)
            .send()
            .await
            .map_err(DNSimpleError::parse_reqwest_error)?;
        self.process_response::<E>(response).await
    }

    async fn call<E: Endpoint>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<DNSimpleResponse<E::Output>, DNSimpleError> {
        let response = request
            .send()
            .await
            .map_err(DNSimpleError::parse_reqwest_error)?;
        self.process_response::<E>(response).await
    }

    async fn process_response<E: Endpoint>(
        &self,
        response: reqwest::Response,
    ) -> Result<DNSimpleResponse<E::Output>, DNSimpleError> {
        let status = response.status().as_u16();

        if response.status().is_success() {
            Self::build_dnsimple_response::<E>(response).await
        } else {
            let body = response.json::<Value>().await.ok();
            Err(DNSimpleError::parse_response(status, body))
        }
    }

    async fn call_empty(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let response = request
            .send()
            .await
            .map_err(DNSimpleError::parse_reqwest_error)?;

        let status = response.status().as_u16();

        if response.status().is_success() {
            Self::build_empty_dnsimple_response(response).await
        } else {
            let body = response.json::<Value>().await.ok();
            Err(DNSimpleError::parse_response(status, body))
        }
    }

    async fn build_dnsimple_response<E: Endpoint>(
        resp: reqwest::Response,
    ) -> Result<DNSimpleResponse<E::Output>, DNSimpleError> {
        let rate_limit = Self::extract_rate_limit_limit_header(&resp)?;
        let rate_limit_remaining = Self::extract_rate_limit_remaining_header(&resp)?;
        let rate_limit_reset = Self::extract_rate_limit_reset_header(&resp)?;

        let status = resp.status().as_u16();

        // if the response is empty, we return empty data
        if status == 204 {
            return Ok(DNSimpleResponse {
                rate_limit,
                rate_limit_remaining,
                rate_limit_reset,
                status,
                data: None,
                pagination: None,
                body: None,
            });
        }

        let json = resp
            .json::<Value>()
            .await
            .map_err(|e| DNSimpleError::Deserialization(e.to_string()))?;
        let data = serde_json::from_value(json!(json.get("data")))
            .map_err(|e| DNSimpleError::Deserialization(e.to_string()))?;
        let pagination = serde_json::from_value(json!(json.get("pagination")))
            .map_err(|e| DNSimpleError::Deserialization(e.to_string()))?;
        let body = serde_json::from_value(json)
            .map_err(|e| DNSimpleError::Deserialization(e.to_string()))?;

        Ok(DNSimpleResponse {
            rate_limit,
            rate_limit_remaining,
            rate_limit_reset,
            status,
            data,
            pagination,
            body,
        })
    }

    fn extract_rate_limit_reset_header(resp: &reqwest::Response) -> Result<String, DNSimpleError> {
        match resp.headers().get("X-RateLimit-Reset") {
            Some(header) => header.to_str().map(|s| s.to_string()).map_err(|_| {
                DNSimpleError::Deserialization(String::from(
                    "Cannot parse the X-RateLimit-Reset header",
                ))
            }),
            None => Err(DNSimpleError::Deserialization(String::from(
                "Cannot parse the X-RateLimit-Reset header",
            ))),
        }
    }

    fn extract_rate_limit_remaining_header(
        resp: &reqwest::Response,
    ) -> Result<String, DNSimpleError> {
        match resp.headers().get("X-RateLimit-Remaining") {
            Some(header) => header.to_str().map(|s| s.to_string()).map_err(|_| {
                DNSimpleError::Deserialization(String::from(
                    "Cannot parse the X-RateLimit-Remaining header",
                ))
            }),
            None => Err(DNSimpleError::Deserialization(String::from(
                "Cannot parse the X-RateLimit-Remaining header",
            ))),
        }
    }

    fn extract_rate_limit_limit_header(resp: &reqwest::Response) -> Result<String, DNSimpleError> {
        match resp.headers().get("X-RateLimit-Limit") {
            Some(header) => header.to_str().map(|s| s.to_string()).map_err(|_| {
                DNSimpleError::Deserialization(String::from(
                    "Cannot parse the X-RateLimit-Limit header",
                ))
            }),
            None => Err(DNSimpleError::Deserialization(String::from(
                "Cannot parse the X-RateLimit-Limit header",
            ))),
        }
    }

    async fn build_empty_dnsimple_response(
        response: reqwest::Response,
    ) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        Ok(DNSimpleEmptyResponse {
            rate_limit: Self::extract_rate_limit_limit_header(&response)?,
            rate_limit_remaining: Self::extract_rate_limit_remaining_header(&response)?,
            rate_limit_reset: Self::extract_rate_limit_reset_header(&response)?,
            status: response.status().as_u16(),
        })
    }

    fn build_get_request(
        &self,
        path: &str,
        options: Option<RequestOptions>,
    ) -> reqwest::RequestBuilder {
        let mut query_params: Vec<(String, String)> = Vec::new();

        if let Some(options) = options {
            if let Some(pagination) = options.paginate {
                query_params.push(("page".to_string(), pagination.page.to_string()));
                query_params.push(("per_page".to_string(), pagination.per_page.to_string()));
            }

            if let Some(filters) = options.filters {
                for (key, value) in filters.filters {
                    query_params.push((key, value));
                }
            }

            if let Some(sort) = options.sort {
                query_params.push(("sort".to_string(), sort.sort_by));
            }
        }

        let request = self
            ._client
            .get(self.url(path))
            .header("User-Agent", &self.user_agent)
            .header("Accept", "application/json")
            .query(&query_params);

        self.add_headers_to_request(request)
    }

    pub fn build_post_request(&self, path: &str) -> reqwest::RequestBuilder {
        let request = self
            ._client
            .post(self.url(path))
            .header("User-Agent", &self.user_agent)
            .header("Accept", "application/json");
        self.add_headers_to_request(request)
    }

    pub fn build_put_request(&self, path: &str) -> reqwest::RequestBuilder {
        let request = self
            ._client
            .put(self.url(path))
            .header("User-Agent", &self.user_agent)
            .header("Accept", "application/json");
        self.add_headers_to_request(request)
    }

    pub fn build_patch_request(&self, path: &str) -> reqwest::RequestBuilder {
        let request = self
            ._client
            .patch(self.url(path))
            .header("User-Agent", &self.user_agent)
            .header("Accept", "application/json");
        self.add_headers_to_request(request)
    }

    fn build_delete_request(&self, path: &str) -> reqwest::RequestBuilder {
        let request = self
            ._client
            .delete(self.url(path))
            .header("User-Agent", &self.user_agent)
            .header("Accept", "application/json");
        self.add_headers_to_request(request)
    }

    fn add_headers_to_request(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        let auth_token = format!("Bearer {}", self.auth_token);
        request.header("Authorization", auth_token.as_str())
    }

    pub fn url(&self, path: &str) -> String {
        let mut url = self.versioned_url();
        url.push_str(path);
        url
    }
}

#[cfg(test)]
mod tests {
    use crate::dnsimple::{new_client, DEFAULT_SANDBOX_URL, DEFAULT_USER_AGENT, VERSION};

    #[test]
    fn creates_a_client() {
        let token = "some-auth-token";
        let client = new_client(true, String::from(token));

        assert_eq!(client.base_url, DEFAULT_SANDBOX_URL);
        assert_eq!(client.user_agent, DEFAULT_USER_AGENT.to_owned() + VERSION);
        assert_eq!(client.auth_token, token);
    }

    #[test]
    fn can_change_the_base_url() {
        let mut client = new_client(true, String::from("token"));
        client.set_base_url("https://example.com");

        assert_eq!(client.versioned_url(), "https://example.com/v2");
    }
}
