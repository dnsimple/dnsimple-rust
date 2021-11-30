use crate::dnsimple::{Client, DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, RequestOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Webhook {
    pub id: u64,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct WebhookPayload {
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

pub struct Webhooks<'a> {
    pub client: &'a Client
}

impl Webhooks<'_> {

    /// List webhooks in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account id
    pub fn list_webhooks(&self, account_id: u64, options: Option<RequestOptions>) -> Result<DNSimpleResponse<Vec<Webhook>>, String> {
        let path = format!("/{}/webhooks", account_id);

        self.client.get::<WebhooksEndpoint>(&path, options)
    }

    /// Create a webhook in the account
    ///
    /// # Arguments
    ///
    /// `account_id`: The account id
    /// `url`: The webhook url
    pub fn create_webhook(&self, account_id: u64, url: String) -> Result<DNSimpleResponse<Webhook>, String> {
        let path = format!("/{}/webhooks", account_id);
        let payload = WebhookPayload { url };

        self.client.post::<WebhookEndpoint>(&path, serde_json::to_value(payload).unwrap())
    }

    /// Retrieve a webhook
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `webhook`: The webhook id
    pub fn get_webhook(&self, account_id: u64, webhook: String) -> Result<DNSimpleResponse<Webhook>, String> {
        let path = format!("/{}/webhooks/{}", account_id, webhook);

        self.client.get::<WebhookEndpoint>(&path, None)
    }

    /// Deletes a webhook
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `webhook`: The webhook id
    pub fn delete_webhook(&self, account_id: u64, webhook: String) -> DNSimpleEmptyResponse {
        let path = format!("/{}/webhooks/{}", account_id, webhook);

        self.client.delete(&path)
    }
}