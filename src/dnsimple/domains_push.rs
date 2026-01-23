use crate::dnsimple::domains::Domains;
use crate::dnsimple::{DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, RequestOptions};
use crate::errors::DNSimpleError;
use serde::{Deserialize, Serialize};

struct DomainPushesListEndpoint;

impl Endpoint for DomainPushesListEndpoint {
    type Output = Vec<DomainPush>;
}

struct DomainPushEndpoint;

impl Endpoint for DomainPushEndpoint {
    type Output = DomainPush;
}

/// Represents a domain push
#[derive(Debug, Deserialize, Serialize)]
pub struct DomainPush {
    /// The domain push ID in DNSimple.
    pub id: u64,
    /// The associated domain ID.
    pub domain_id: u64,
    /// The associated contact ID.
    pub contact_id: Option<u64>,
    /// The associated account ID.
    pub account_id: u64,
    /// When the domain push was created in DNSimple.
    pub created_at: String,
    /// When the domain push was last updated in DNSimple.
    pub updated_at: String,
    /// When the domain push was accepted in DNSimple.
    pub accepted_at: Option<String>,
}

/// Payload to initiate a push
#[derive(Debug, Deserialize, Serialize)]
pub struct InitiatePushPayload {
    /// The email address of the target DNSimple account.
    pub new_account_email: String,
}

/// The domains push set of endpoints
///
/// See [API Documentation: domains/pushes](https://developer.dnsimple.com/v2/domains/pushes)
impl Domains<'_> {
    /// Initiate a push
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::domains_push::InitiatePushPayload;
    /// use dnsimple::dnsimple::new_client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = new_client(true, String::from("AUTH_TOKEN"));
    ///     let payload = InitiatePushPayload {
    ///         new_account_email: String::from("admin@target-account.test"),
    ///     };
    ///     let push = client.domains().initiate_push(1234, "example.com", payload).await.unwrap().data.unwrap();
    /// }
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account id
    /// `domain`: The domain name or id
    /// `payload`: The `InitiatePushPayload` used to initiate a push
    pub async fn initiate_push(
        &self,
        account_id: u64,
        domain: &str,
        payload: InitiatePushPayload,
    ) -> Result<DNSimpleResponse<DomainPush>, DNSimpleError> {
        let path = format!("/{}/domains/{}/pushes", account_id, domain);

        match serde_json::to_value(payload) {
            Ok(json) => self.client.post::<DomainPushEndpoint>(&path, json).await,
            Err(_) => Err(DNSimpleError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }

    /// List pending pushes for the target account.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = new_client(true, String::from("AUTH_TOKEN"));
    ///     let pushes = client.domains().list_pushes(1234, None).await.unwrap().data.unwrap();
    /// }
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account id
    /// `options`: The `RequestOptions`
    ///            - Pagination
    pub async fn list_pushes(
        &self,
        account_id: u64,
        options: Option<RequestOptions>,
    ) -> Result<DNSimpleResponse<Vec<DomainPush>>, DNSimpleError> {
        let path = format!("/{}/domains/pushes", account_id);

        self.client.get::<DomainPushesListEndpoint>(&path, options).await
    }

    /// Accept a push
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = new_client(true, String::from("AUTH_TOKEN"));
    ///     let response = client.domains().accept_push(1234, 42).await;
    /// }
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account id
    /// `push_id`: The push id
    pub async fn accept_push(
        &self,
        account_id: u64,
        push_id: u64,
    ) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let path = format!("/{}/domains/pushes/{}", account_id, push_id);

        self.client.empty_post(&path).await
    }

    /// Reject a push
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = new_client(true, String::from("AUTH_TOKEN"));
    ///     let response = client.domains().reject_push(1234, 42).await;
    /// }
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account id
    /// `push_id`: The push id
    pub async fn reject_push(
        &self,
        account_id: u64,
        push_id: u64,
    ) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let path = format!("/{}/domains/pushes/{}", account_id, push_id);

        self.client.delete(&path).await
    }
}
