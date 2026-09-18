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
    /// See [API Documentation](https://developer.dnsimple.com/v2/webhooks/webhooks/#listWebhooks)
    ///
    /// # Arguments
    ///
    /// `account_id`: The account id
    pub async fn list_webhooks(
        &self,
        account_id: u64,
        options: Option<RequestOptions>,
    ) -> Result<DNSimpleResponse<Vec<Webhook>>, DNSimpleError> {
        let path = format!("/{}/webhooks", account_id);

        self.client.get::<WebhooksEndpoint>(&path, options).await
    }

    /// Create a webhook in the account
    ///
    /// See [API Documentation](https://developer.dnsimple.com/v2/webhooks/webhooks/#createWebhook)
    ///
    /// # Arguments
    ///
    /// `account_id`: The account id
    /// `url`: The webhook url
    pub async fn create_webhook(
        &self,
        account_id: u64,
        url: String,
    ) -> Result<DNSimpleResponse<Webhook>, DNSimpleError> {
        let path = format!("/{}/webhooks", account_id);
        let payload = WebhookPayload { url };

        self.client.post::<WebhookEndpoint>(&path, payload).await
    }

    /// Retrieve a webhook
    ///
    /// See [API Documentation](https://developer.dnsimple.com/v2/webhooks/webhooks/#getWebhook)
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `webhook`: The webhook id
    pub async fn get_webhook(
        &self,
        account_id: u64,
        webhook: String,
    ) -> Result<DNSimpleResponse<Webhook>, DNSimpleError> {
        let path = format!("/{}/webhooks/{}", account_id, webhook);

        self.client.get::<WebhookEndpoint>(&path, None).await
    }

    /// Deletes a webhook
    ///
    /// See [API Documentation](https://developer.dnsimple.com/v2/webhooks/webhooks/#deleteWebhook)
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `webhook`: The webhook id
    pub async fn delete_webhook(
        &self,
        account_id: u64,
        webhook: String,
    ) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let path = format!("/{}/webhooks/{}", account_id, webhook);

        self.client.delete(&path).await
    }
}
