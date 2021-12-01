use std::collections::HashMap;
use serde;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use ureq::{Error, Request, Response};
use crate::dnsimple::accounts::Accounts;
use crate::dnsimple::certificates::Certificates;
use crate::dnsimple::contacts::Contacts;
use crate::dnsimple::domains::Domains;
use crate::dnsimple::identity::Identity;
use crate::dnsimple::registrar::Registrar;
use crate::dnsimple::services::Services;
use crate::dnsimple::templates::Templates;
use crate::dnsimple::tlds::Tlds;
use crate::dnsimple::vanity_name_servers::VanityNameServers;
use crate::dnsimple::webhooks::Webhooks;
use crate::dnsimple::zones::Zones;

pub mod identity;
pub mod accounts;
pub mod domains;
pub mod domains_collaborators;
pub mod domains_dnssec;
pub mod domains_signer_records;
pub mod domains_email_forwards;
pub mod domains_push;
pub mod certificates;
pub mod tlds;
pub mod registrar;
pub mod registrar_name_servers;
pub mod registrar_auto_renewal;
pub mod registrar_whois_privacy;
pub mod zones;
pub mod zones_records;
pub mod contacts;
pub mod services;
pub mod templates;
pub mod vanity_name_servers;
pub mod webhooks;

const VERSION: &str = "0.1.0";
const DEFAULT_USER_AGENT: &str = "dnsimple-rust/";

const API_VERSION: &str = "v2";
const DEFAULT_BASE_URL: &str = "https://api.dnsimple.com";
const DEFAULT_SANDBOX_URL: &str  = "https://api.sandbox.dnsimple.com";

/// Represents the Rust client for the DNSimple API V2
///
/// The client is your entrypoint to the DNSimple API. Using it
/// you will be able to call all the endpoints of the DNSimple API
/// and their respective functions.
///
/// # Examples
///
/// ```no_run
/// use dnsimple_rust::dnsimple::{Client, new_client};
///
/// let client = new_client(true, String::from("AUTH_TOKEN"));
/// let identity = client.identity().whoami().unwrap().data.unwrap();
///
/// let account = identity.account.unwrap();
/// ```
///
pub struct Client {
    base_url: String,
    user_agent: String,
    auth_token: String,
    pub _agent: ureq::Agent,
}

/// Represents the Error message payload returned by the DNSimple API
#[derive(Debug, Deserialize, Serialize)]
pub struct APIErrorMessage {
    pub message: Option<String>,
    pub errors: Option<Value>,
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
    /// The object or a Vec<T> of objects (the type `T` will depend on the endpoint).
    pub data: Option<T>,
    /// The error response if any
    pub errors: Option<APIErrorMessage>,
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
    /// Filtering makes it possible to ask only for the exact subset of data that you you’re looking for.
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

/// Filtering makes it possible to ask only for the exact subset of data that you you’re looking for.
//
// With potential hundreds of result entries, it’s convenient to apply a filter and receive only the
// interesting data.
#[derive(Debug)]
pub struct Filters {
    pub filters: HashMap<String, String>
}

impl Filters {
    pub fn new(filters: HashMap<String, String>) -> Filters {
        Filters{ filters }
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
    pub sort_by: String
}

impl Sort {
    pub fn new(sort_by: String) -> Sort {
        Sort{ sort_by }
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
/// use dnsimple_rust::dnsimple::{Client, new_client};
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
        _agent: ureq::Agent::new(),
    }
}

impl Client {
    ///Returns the `accounts` service attached to this client
    pub fn accounts(&self) -> Accounts {
        Accounts {
            client: self
        }
    }

    /// Returns the `contacts` service attached to this client
    pub fn contacts(&self) -> Contacts {
        Contacts {
            client: self
        }
    }

    /// Returns the `certificates` service attached to this client
    pub fn certificates(&self) -> Certificates {
        Certificates {
            client: self
        }
    }

    /// Returns the `domains` service attached to this client
    pub fn domains(&self) -> Domains {
        Domains {
            client: self
        }
    }

    /// Returns the `identity` service attached to this client
    pub fn identity(&self) -> Identity {
        Identity {
            client: self
        }
    }

    /// Returns the `registrar` service attached to this client
    pub fn registrar(&self) -> Registrar {
        Registrar {
            client: self
        }
    }

    /// Returns the `services` service attached to this client
    pub fn services(&self) -> Services {
        Services {
            client: self
        }
    }

    /// Returns the `templates` service attached to this client
    pub fn templates(&self) -> Templates {
        Templates {
            client: self
        }
    }

    /// Returns the `tlds` service attached to this endpoint
    pub fn tlds(&self) -> Tlds {
        Tlds {
            client: self
        }
    }

    /// Returns the `vanity_name_servers` service attached to this endpoint
    pub fn vanity_name_servers(&self) -> VanityNameServers {
        VanityNameServers {
            client: self
        }
    }

    /// Returns the `webhooks` service attached to this endpoint
    pub fn webhooks(&self) -> Webhooks {
        Webhooks {
            client: self
        }
    }

    /// Returns the `zones` service attached to this endpoint
    pub fn zones(&self) -> Zones {
        Zones {
            client: self
        }
    }

    /// Convenience function to change the base url in runtime (used internally for
    /// testing).
    ///
    /// Note that if you want to do this you will have to declare your client mutable.
    ///
    /// ```no_run
    /// use dnsimple_rust::dnsimple::{Client, new_client};
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
        url.push_str("/");
        url.push_str(API_VERSION);
        url
    }

    /// Sends a GET request to the DNSimple API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `options`: optionally a `RequestOptions` with things like pagination, filtering and sorting
    pub fn get<E: Endpoint>(&self, path: &str, options: Option<RequestOptions>) -> Result<DNSimpleResponse<E::Output>, String> {
        self.call::<E>(self.build_get_request(&path, options))
    }

    /// Sends a POST request to the DNSimple API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `data`: the json payload to be sent to the server
    pub fn post<E: Endpoint>(&self, path: &str, data: Value) -> Result<DNSimpleResponse<E::Output>, String> {
        self.call_with_payload::<E>(self.build_post_request(&path), data)
    }

    /// Sends a POST request to the DNSimple API without any payload
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub fn empty_post(&self, path: &str) -> DNSimpleEmptyResponse {
        self.call_empty(self.build_post_request(&path))
    }

    /// Sends a PUT request to the DNSimple API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `data`: the json payload to be sent to the server
    pub fn put<E: Endpoint>(&self, path: &str, data: Value) -> Result<DNSimpleResponse<E::Output>, String> {
        self.call_with_payload::<E>(self.build_put_request(&path), data)
    }

    /// Sends a PUT request to the DNSimple API without any payload
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub fn empty_put(&self, path: &str) -> DNSimpleEmptyResponse {
        self.call_empty(self.build_put_request(&path))
    }

    /// Sends a PATCH request to the DNSimple API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `data`: the json payload to be sent to the server
    pub fn patch<E: Endpoint>(&self, path: &str, data: Value) -> Result<DNSimpleResponse<E::Output>, String> {
        self.call_with_payload::<E>(self.build_patch_request(&path), data)
    }

    /// Sends a DELETE request to the DNSimple API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub fn delete(&self, path: &str) -> DNSimpleEmptyResponse {
        self.call_empty(self.build_delete_request(&path))
    }

    /// Sends a DELETE request to the DNSimple API returning a response containing a `DNSimpleResponse`
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub fn delete_with_response<E: Endpoint>(&self, path: &str) -> Result<DNSimpleResponse<E::Output>, String> {
        self.call::<E>(self.build_delete_request(&path))
    }

    fn call_with_payload<E: Endpoint>(&self, request: Request, data: Value) -> Result<DNSimpleResponse<E::Output>, String> {
        match request.send_json(data) {
            Ok(response) => {
                Self::build_dnsimple_response::<E>(response)
            },
            Err(Error::Status(_code, response)) => {
                Self::build_dnsimple_response::<E>(response)
            },
            Err(_) => { panic!("Something went really wrong!")}
        }
    }

    fn call<E: Endpoint>(&self, request: Request) -> Result<DNSimpleResponse<E::Output>, String> {
        match request.call() {
            Ok(response) => {
                Self::build_dnsimple_response::<E>(response)
            },
            Err(Error::Status(_code, response)) => {
                Self::build_dnsimple_response::<E>(response)
            },
            Err(_) => { panic!("Something went really wrong!")}
        }
    }

    fn call_empty(&self, request: Request) -> DNSimpleEmptyResponse {
        match request.call() {
            Ok(response) => {
                Self::build_empty_dnsimple_response(response)
            },
            Err(Error::Status(_code, response)) => {
                Self::build_empty_dnsimple_response(response)
            },
            Err(_) => { panic!("Something went really wrong!")}
        }
    }

    fn build_dnsimple_response<E: Endpoint>(resp: Response) -> Result<DNSimpleResponse<E::Output>, String> {
        let rate_limit= String::from(resp.header("X-RateLimit-Limit").unwrap());
        let rate_limit_remaining= String::from(resp.header("X-RateLimit-Remaining").unwrap());
        let rate_limit_reset= String::from(resp.header("X-RateLimit-Reset").unwrap());
        let status= resp.status();

        let json = resp.into_json::<Value>().unwrap();
        let data = serde_json::from_value(json!(json.get("data"))).map_err(|e| e.to_string())?;
        let errors = serde_json::from_value(json!(json)).map_err(|e| e.to_string())?;
        let pagination = serde_json::from_value(json!(json.get("pagination"))).map_err(|e| e.to_string())?;
        let body = serde_json::from_value(json).map_err(|e| e.to_string())?;

        Ok(DNSimpleResponse {
            rate_limit, rate_limit_remaining, rate_limit_reset, status,
            data,
            errors, pagination, body,
        })
    }

    fn build_empty_dnsimple_response(response: Response) -> DNSimpleEmptyResponse {
        DNSimpleEmptyResponse {
            rate_limit: String::from(response.header("X-RateLimit-Limit").unwrap()),
            rate_limit_remaining: String::from(response.header("X-RateLimit-Remaining").unwrap()),
            rate_limit_reset: String::from(response.header("X-RateLimit-Reset").unwrap()),
            status: response.status(),
        }
    }

    fn build_get_request(&self, path: &&str, options: Option<RequestOptions>) -> Request {
        let mut request = self._agent.get(&*self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json");

        match options {
            Some(options) => {
                match options.paginate {
                    Some(pagination) =>  {
                        request = request.query("page", &*pagination.page.to_string());
                        request = request.query("per_page", &*pagination.per_page.to_string())
                    }
                    _ => {}
                }
                match options.filters {
                    Some(filters) => {
                        for(key, value) in filters.filters {
                            request = request.query(&*key, &*value);
                        }
                    }
                    _ => {}
                }
                match options.sort {
                    Some(sort) => {
                        request = request.query("sort", &*sort.sort_by);
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        self.add_headers_to_request(request.to_owned())
    }

    pub fn build_post_request(&self, path: &&str) -> Request {
        self._agent.post(&self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json")
    }

    pub fn build_put_request(&self, path: &&str) -> Request {
        self._agent.put(&self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json")
    }

    pub fn build_patch_request(&self, path: &&str) -> Request {
        self._agent.request("PATCH", &self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json")
    }

    fn build_delete_request(&self, path: &&str) -> Request {
        let request = self._agent.delete(&self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json");
        self.add_headers_to_request(request)
    }

    fn add_headers_to_request(&self, request: Request) -> Request {
        let auth_token = &format!("Bearer {}", self.auth_token);
        request
            .set("Authorization", auth_token.as_str())
    }

    fn url(&self, path: &str) -> String {
        let mut url = self.versioned_url();
        url.push_str(path);
        url
    }
}

#[cfg(test)]
mod tests {
    use crate::dnsimple::{DEFAULT_SANDBOX_URL, DEFAULT_USER_AGENT, new_client, VERSION};

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