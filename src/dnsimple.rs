use std::collections::HashMap;
use serde;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use ureq::{Error, Request, Response};
use crate::dnsimple::accounts::Accounts;
use crate::dnsimple::domains::Domains;
use crate::dnsimple::identity::Identity;
use crate::dnsimple::certificates::Certificates;
use crate::dnsimple::tlds::Tlds;
use crate::dnsimple::registrar::Registrar;

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
/// // use dnsimple_rust::dnsimple::{Client, new_client};
///
/// // let client = new_client(true, String::from("AUTH_TOKEN"));
/// // let identity_response = client.identity().whoami().data;
///
/// // match identity_response {
/// //         None => panic!("We should have a payload here."),
/// //         Some(whoami) =>  match whoami.data.account {
/// //             None => panic!("We should have the account data here"),
/// //             Some(account) => {
///             // so something with the account, like retrieving the id
///             // with account.id
///             // }
///         // }
/// // }
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

pub trait Endpoint {
    type Output: DeserializeOwned;
}

#[derive(Debug)]
pub struct DNSimpleResponse<T> {
    pub rate_limit: String,
    pub rate_limit_remaining: String,
    pub rate_limit_reset: String,
    pub status: u16,
    pub data: Option<T>,
    pub errors: Option<APIErrorMessage>,
    pub pagination: Option<Pagination>,
    pub body: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    current_page: u64,
    per_page: u64,
    total_entries: u64,
    total_pages: u64,
}

/// Represents an empty response from the DNSimple API
/// (_these type of responses happen when issuing DELETE commands for example_)
pub struct DNSimpleEmptyResponse {
    pub rate_limit: String,
    pub rate_limit_remaining: String,
    pub rate_limit_reset: String,
    pub status: u16,
}

#[derive(Debug)]
pub struct Filters {
    pub filters: HashMap<String, String>
}

impl Filters {
    pub fn new(filters: HashMap<String, String>) -> Filters {
        Filters{ filters }
    }
}

#[derive(Debug)]
pub struct Sort {
    pub sort_by: String
}

impl Sort {
    pub fn new(sort_by: String) -> Sort {
        Sort{ sort_by }
    }
}

pub struct Paginate {
    pub per_page: u32,
    pub page: u32,
}

/// Wrapper around a DNSimpleResponse and the raw http response of the DNSimple API
// pub struct APIResponse<T> {
//     pub response: DNSimpleResponse <T>,
//     pub raw_http_response: Response
// }

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

    /// Returns the `tlds` service attached to this endpoint
    pub fn tlds(&self) -> Tlds {
        Tlds {
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
    pub fn get<E: Endpoint>(&self, path: &str, filters: Filters, sort: Sort, paginate: Paginate) -> Result<DNSimpleResponse<E::Output>, String> {
        let request = self.build_get_request(&path, filters, sort, paginate);

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

    /// Sends a POST request to the DNSimple API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `data`: the json payload to be sent to the server
    pub fn post<E: Endpoint>(&self, path: &str, data: Value) -> Result<DNSimpleResponse<E::Output>, String> {
        let request = self.build_post_request(&path);

        match request.send_json(data) {
           Ok(response) => {
               Self::build_dnsimple_response::<E>(response)
           },
           Err(Error::Status(_code, response)) => {
               Self::build_dnsimple_response::<E>(response)
           },
            Err(_) => { panic!("Something went really wong!")}
        }
    }

    /// Sends a DELETE request to the DNSimple API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub fn delete(&self, path: &str) -> DNSimpleEmptyResponse {
        let request = self.build_delete_request(&path);
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

    pub fn empty_post(&self, path: &str) -> DNSimpleEmptyResponse {
        let request = self.build_post_request(&path);
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

    // TODO: remove the '_' from filters, sort and paginate once you've figured out how to do 295
    fn build_get_request(&self, path: &&str, _filters: Filters, _sort: Sort, _paginate: Paginate) -> Request {

        let request = self._agent.get(&*self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json");

        // TODO: figure out how to add the query (with filters and sort to the request)
        self.add_headers_to_request(request.to_owned())
    }

    pub fn build_post_request(&self, path: &&str) -> Request {
        self._agent.post(&self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json")
    }

    fn build_delete_request(&self, path: &&str) -> Request {
        let request = self._agent.delete(&self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json");
        self.add_headers_to_request(request)
    }

    // TODO: see build_get_request ...
    // fn add_query_to_request(&self, request: &Request, filters: Filters, sort: Sort, paginate: Paginate) -> Request {
    //     for (key, value) in filters.filters.into_iter() {
    //         request.query(&*key, &*value);
    //     }
    //     if !sort.sort_by.is_empty() {
    //         request.query("sort", &*sort.sort_by);
    //     }
    //     request
    // }

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