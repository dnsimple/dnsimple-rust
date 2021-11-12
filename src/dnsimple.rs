use std::iter::Map;
use ureq::{Request, Response};
use crate::dnsimple::identity::Identity;

pub mod identity;

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
/// let identity_response = client.identity().whoami().data;
///
/// match identity_response {
///         None => panic!("We should have a payload here."),
///         Some(whoami) =>  match whoami.data.account {
///             None => panic!("We should have the account data here"),
///             Some(account) => {
///             // so something with the account, like retrieving the id
///             // with account.id
///             }
///         }
/// }
pub struct Client {
    base_url: String,
    user_agent: String,
    auth_token: String,
    agent: ureq::Agent,
}

/// Represents the Error message payload returned by the DNSimple API
pub struct APIErrorMessage {
    pub message: String,
    pub errors: Map<String, Vec<String>>
}

/// Represents a response from the DNSimple API
pub struct DNSimpleResponse<T> {
    pub rate_limit: String,
    pub rate_limit_remaining: String,
    pub rate_limit_reset: String,
    pub status: u16,
    pub data: Option<T>,
    pub message: Option<APIErrorMessage>
}

/// Wrapper around a DNSimpleResponse and the raw http response of the DNSimple API
pub struct APIResponse<T> {
    pub response: DNSimpleResponse <T>,
    pub raw_http_response: Response
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
        agent: ureq::Agent::new(),
    }
}

impl Client {
    /// Returns the `identity` service attached to this client
    pub fn identity(&self) -> Identity {
        Identity {
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
    pub fn get<T>(&self, path: &str) -> APIResponse<T> {
        let request = self.build_get_request(&path);

        let response = request.call().unwrap();
        let dnsimple_response = DNSimpleResponse {
            rate_limit: String::from(response.header("X-RateLimit-Limit").unwrap()),
            rate_limit_remaining: String::from(response.header("X-RateLimit-Remaining").unwrap()),
            rate_limit_reset: String::from(response.header("X-RateLimit-Reset").unwrap()),
            status: response.status(),
            data: None,
            message: None
        };

        APIResponse {
            response: dnsimple_response,
            raw_http_response: response,
        }
    }

    fn build_get_request(&self, path: &&str) -> Request {
        let request = self.agent.get(&self.url(path)).set("User-Agent", &self.user_agent);
        self.add_headers_to_request(request)
    }

    fn add_headers_to_request(&self, request: Request) -> Request {
        let auth_token = &format!("Bearer {}", self.auth_token);
        request
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json")
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