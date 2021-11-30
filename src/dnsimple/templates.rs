use crate::dnsimple::{Client, DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, RequestOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Template {
    pub id: u64,
    pub account_id: u64,
    pub name: String,
    pub sid: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplatePayload {
    pub name: String,
    pub sid: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateRecord {
    pub id: u64,
    pub template_id: u64,
    pub name: String,
    pub content: String,
    pub ttl: u64,
    pub priority: Option<u64>,
    #[serde(rename = "type")]
    pub record_type: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateRecordPayload {
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub content: String,
    pub ttl: Option<u64>,
    pub priority: Option<u64>,
}

struct TemplatesEndpoint;

impl Endpoint for TemplatesEndpoint {
    type Output = Vec<Template>;
}

struct TemplateEndpoint;

impl Endpoint for TemplateEndpoint {
    type Output = Template;
}

struct TemplateRecordsEndpoint;

impl Endpoint for TemplateRecordsEndpoint {
    type Output = Vec<TemplateRecord>;
}

struct TemplateRecordEndpoint;

impl Endpoint for TemplateRecordEndpoint {
    type Output = TemplateRecord;
}

pub struct Templates<'a> {
    pub client: &'a Client
}

impl Templates<'_> {

    /// List templates in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account id
    /// `options`: The `RequestOptions` for sorting, etc.
    pub fn list_templates(&self, account_id: u64, options: Option<RequestOptions>) -> Result<DNSimpleResponse<Vec<Template>>, String> {
        let path = format!("/{}/templates", account_id);

        self.client.get::<TemplatesEndpoint>(&path, options)
    }

    /// Create a template in the account
    ///
    /// # Arguments
    ///
    /// `account_id`: The account id
    /// `payload`: The `Template payload` with the information to create the template
    pub fn create_template(&self, account_id: u64, payload: TemplatePayload) -> Result<DNSimpleResponse<Template>, String> {
        let path = format!("/{}/templates", account_id);

        self.client.post::<TemplateEndpoint>(&path, serde_json::to_value(payload).unwrap())
    }

    /// Retrieve a template in the account
    ///
    /// # Arguments
    ///
    /// `account_id`: The account id
    /// `template`: The template name or id
    pub fn get_template(&self, account_id: u64, template: String) -> Result<DNSimpleResponse<Template>,String> {
        let path = format!("/{}/templates/{}", account_id, template);

        self.client.get::<TemplateEndpoint>(&path, None)
    }

    /// Update a template in the account
    ///
    /// # Arguments
    ///
    /// `account_id`: The account id
    /// `template`: The template name or id
    /// `payload`: The `Template payload` with the information to create the template
    pub fn update_template(&self, account_id: u64, template: String, payload: TemplatePayload) -> Result<DNSimpleResponse<Template>, String> {
        let path = format!("/{}/templates/{}", account_id, template);

        self.client.patch::<TemplateEndpoint>(&path, serde_json::to_value(payload).unwrap())
    }

    /// Deletes a template from the account
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `template`: The template name or id
    pub fn delete_template(&self, account_id: u64, template: String) -> DNSimpleEmptyResponse {
        let path = format!("/{}/templates/{}", account_id, template);

        self.client.delete(&path)
    }

    /// List template records
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `template`: The template name or id
    pub fn list_template_records(&self, account_id: u64, template: String, options: Option<RequestOptions>) -> Result<DNSimpleResponse<Vec<TemplateRecord>>, String> {
        let path = format!("/{}/templates/{}/records", account_id, template);

        self.client.get::<TemplateRecordsEndpoint>(&path, options)
    }

    /// Create a template record
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `template`: The template name or id
    /// `payload`: The `TemplateRecordPayload` with the information needed to create the template record
    pub fn create_template_record(&self, account_id: u64, template: String, payload: TemplateRecordPayload) -> Result<DNSimpleResponse<TemplateRecord>, String> {
        let path = format!("/{}/templates/{}/records", account_id, template);

        self.client.post::<TemplateRecordEndpoint>(&path, serde_json::to_value(payload).unwrap())
    }

    /// Retrieve a template record
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `template`: The template name or id
    /// `record`: The record id
    pub fn get_template_record(&self, account_id: u64, template: String, record: u64) -> Result<DNSimpleResponse<TemplateRecord>, String> {
        let path = format!("/{}/templates/{}/records/{}", account_id, template, record);

        self.client.get::<TemplateRecordEndpoint>(&path, None)
    }

    /// Delete a template record
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `template`: The template name or id
    /// `record`: The record id
    pub fn delete_template_record(&self, account_id: u64, template: String, record: u64) -> DNSimpleEmptyResponse {
        let path = format!("/{}/templates/{}/records/{}", account_id, template, record);

        self.client.delete(&path)
    }

    /// Applies a template to a domain.
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `domain`: The domain name or id
    /// `template`: The template id or short name
    pub fn apply_template(&self, account_id: u64, domain: String, template: String) -> DNSimpleEmptyResponse {
        let path = format!("/{}/domains/{}/templates/{}", account_id, domain, template);

        self.client.empty_post(&path)
    }
}
