use crate::dnsimple::{Client, DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, RequestOptions};
use crate::errors::DNSimpleError;
use serde::{Deserialize, Serialize};

/// Represents a webhook
#[derive(Debug, Deserialize)]
pub struct Webhook {
    /// The webhook ID in DNSimple.
    pub id: u64,
    /// The callback URL.
    pub url: String,
}

/// Represents the payload to be sent to create a webhook
#[derive(Debug, Serialize)]
pub struct WebhookPayload {
    /// The callback url
    pub url: String,
}

struct WebhooksEndpoint;

impl Endpoint for WebhooksEndpoint {
    type Output = Vec<Webhook>;
}

struct WebhookEndpoint;

impl Endpoint for WebhookEndpoint {
    type Output = Webhook;
}

/// The Webhooks Service handles the webhooks of the DNSimple API.
///
/// See [API Documentation: webhooks](https://developer.dnsimple.com/v2/webhooks/)
pub struct Webhooks<'a> {
    pub client: &'a Client,
}

impl Webhooks<'_> {
    /// List webhooks in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account id
    pub fn list_webhooks(
        &self,
        account_id: u64,
        options: Option<RequestOptions>,
    ) -> Result<DNSimpleResponse<Vec<Webhook>>, DNSimpleError> {
        let path = format!("/{}/webhooks", account_id);

        self.client.get::<WebhooksEndpoint>(&path, options)
    }

    /// Create a webhook in the account
    ///
    /// # Arguments
    ///
    /// `account_id`: The account id
    /// `url`: The webhook url
    pub fn create_webhook(
        &self,
        account_id: u64,
        url: String,
    ) -> Result<DNSimpleResponse<Webhook>, DNSimpleError> {
        let path = format!("/{}/webhooks", account_id);
        let payload = WebhookPayload { url };

        match serde_json::to_value(payload) {
            Ok(json) => self.client.post::<WebhookEndpoint>(&path, json),
            Err(_) => Err(DNSimpleError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }

    /// Retrieve a webhook
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `webhook`: The webhook id
    pub fn get_webhook(
        &self,
        account_id: u64,
        webhook: String,
    ) -> Result<DNSimpleResponse<Webhook>, DNSimpleError> {
        let path = format!("/{}/webhooks/{}", account_id, webhook);

        self.client.get::<WebhookEndpoint>(&path, None)
    }

    /// Deletes a webhook
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `webhook`: The webhook id
    pub fn delete_webhook(
        &self,
        account_id: u64,
        webhook: String,
    ) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let path = format!("/{}/webhooks/{}", account_id, webhook);

        self.client.delete(&path)
    }
}
