use crate::dnsimple::domains::Domains;
use crate::dnsimple::{DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, RequestOptions};
use crate::errors::DNSimpleError;
use serde::{Deserialize, Serialize};

struct EmailForwardsListEndpoint;

impl Endpoint for EmailForwardsListEndpoint {
    type Output = Vec<EmailForwardsInList>;
}

/// Represents an email forwards
#[derive(Debug, Deserialize, Serialize)]
pub struct EmailForward {
    /// The email forward ID in DNSimple.
    pub id: u64,
    /// The associated domain ID.
    pub domain_id: u64,
    /// The email alias
    pub alias_email: String,
    /// The destination email
    pub destination_email: String,
    /// The "local part" of the originating email address. Anything to the left of the @ symbol.
    pub from: String,
    /// The full email address to forward to.
    pub to: String,
    ///  When the email forward was created in DNSimple.
    pub created_at: String,
    /// Then the email forward was last updated in DNSimple.
    pub updated_at: String,
}

/// Represents a shortened email forwards
#[derive(Debug, Deserialize, Serialize)]
pub struct EmailForwardsInList {
    /// The email forward ID in DNSimple.
    pub id: u64,
    /// The domain id
    pub domain_id: u64,
    /// The "local part" of the originating email address. Anything to the left of the @ symbol.
    pub from: String,
    /// The full email address to forward to.
    pub to: String,
    ///  When the email forward was created in DNSimple.
    pub created_at: String,
    /// Then the email forward was last updated in DNSimple.
    pub updated_at: String,
}

/// The payload used to create an email forward
#[derive(Debug, Deserialize, Serialize)]
pub struct EmailForwardPayload {
    /// The alias for this email forwards
    pub alias_name: String,
    /// The destination email
    pub destination_email: String,
}

struct EmailForwardEndpoint;

impl Endpoint for EmailForwardEndpoint {
    type Output = EmailForward;
}

/// The domains email forwards set of endpoints
///
/// See [API Documentation: domains/email-forwards](https://developer.dnsimple.com/v2/domains/email-forwards)
impl Domains<'_> {
    /// List email forwards for the domain in the account.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let email_forwards_list = client.domains().list_email_forwards(1234, "example.com", None).unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want list the email forwards
    /// `options`: The `RequestOptions`
    ///            - Sort: `id`, `from`, `to`
    ///            - Pagination
    pub fn list_email_forwards(
        &self,
        account_id: u64,
        domain: &str,
        options: Option<RequestOptions>,
    ) -> Result<DNSimpleResponse<Vec<EmailForwardsInList>>, DNSimpleError> {
        let path = format!("/{}/domains/{}/email_forwards", account_id, domain);

        self.client.get::<EmailForwardsListEndpoint>(&path, options)
    }

    /// Create an email forward
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::domains_email_forwards::EmailForwardPayload;
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let payload = EmailForwardPayload {
    ///     alias_name: "My forward".to_string(),
    ///     destination_email: "some@example.com".to_string(),
    /// };
    /// let email_forwards = client.domains().create_email_forward(1234, "example.com", payload).unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want list the email forwards
    /// `payload`: The `EmailForwardPayload` with the data needed to create the email forward
    pub fn create_email_forward(
        &self,
        account_id: u64,
        domain: &str,
        payload: EmailForwardPayload,
    ) -> Result<DNSimpleResponse<EmailForward>, DNSimpleError> {
        let path = format!("/{}/domains/{}/email_forwards", account_id, domain);

        match serde_json::to_value(payload) {
            Ok(json) => self.client.post::<EmailForwardEndpoint>(&path, json),
            Err(_) => Err(DNSimpleError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }

    /// Retrieve an email forward
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let email_forwards = client.domains().get_email_forward(1234, "example.com", 42).unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want list the email forwards
    /// `email_forward`: The email forward id
    pub fn get_email_forward(
        &self,
        account_id: u64,
        domain: &str,
        email_forward: u64,
    ) -> Result<DNSimpleResponse<EmailForward>, DNSimpleError> {
        let path = format!(
            "/{}/domains/{}/email_forwards/{}",
            account_id, domain, email_forward
        );

        self.client.get::<EmailForwardEndpoint>(&path, None)
    }

    /// Delete the email forward from the domain.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let response = client.domains().delete_email_forward(1234, "example.com", 42);
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want list the email forwards
    /// `email_forward`: The email forward id
    pub fn delete_email_forward(
        &self,
        account_id: u64,
        domain: &str,
        email_forward: i32,
    ) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let path = format!(
            "/{}/domains/{}/email_forwards/{}",
            account_id, domain, email_forward
        );

        self.client.delete(&path)
    }
}
