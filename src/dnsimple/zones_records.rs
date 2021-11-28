use crate::dnsimple::zones::{Distribution, Zones};
use serde::{Deserialize, Serialize};
use crate::dnsimple::{DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, RequestOptions};
use crate::dnsimple::zones::DistributionEndpoint;


#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneRecord {
    pub id: u64,
    pub zone_id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub content: String,
    pub ttl: u64,
    pub priority: Option<u64>,
    #[serde(rename = "type")]
    pub record_type: String,
    pub regions: Vec<String>,
    pub system_record: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneRecordPayload {
    pub name: String,
    pub record_type: String,
    pub content: String,
    pub ttl: Option<u64>,
    pub priority: Option<u64>,
    pub regions: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneRecordUpdatePayload {
    pub name: Option<String>,
    pub content: Option<String>,
    pub ttl: Option<u64>,
    pub priority: Option<u64>,
    pub regions: Option<Vec<String>>,
}

struct ZoneRecordsEndpoint;

impl Endpoint for ZoneRecordsEndpoint {
    type Output = Vec<ZoneRecord>;
}

struct ZoneRecordEndpoint;

impl Endpoint for ZoneRecordEndpoint {
    type Output = ZoneRecord;
}

impl Zones<'_> {

    // TODO: change all functions that accept String to accept &str?
    /// List zone records
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    pub fn list_zone_records(&self, account_id: u64, zone: &str, options: Option<RequestOptions>) -> Result<DNSimpleResponse<Vec<ZoneRecord>>, String> {
        let path = format!("/{}/zones/{}/records", account_id, zone);

        self.client.get::<ZoneRecordsEndpoint>(&*path, options)
    }

    /// Create a zone record
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    /// `payload`: The `ZoneRecordPayload` with the information to create the zone record
    pub fn create_zone_record(&self, account_id: u64, zone: &str, payload: ZoneRecordPayload) -> Result<DNSimpleResponse<ZoneRecord>, String> {
        let path = format!("/{}/zones/{}/records", account_id, zone);

        self.client.post::<ZoneRecordEndpoint>(&*path, serde_json::to_value(payload).unwrap())
    }

    /// Retrieve a zone record
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    /// `record`: The record id
    pub fn get_zone_record(&self, account_id: u64, zone: &str, record: u64) -> Result<DNSimpleResponse<ZoneRecord>, String> {
        let path = format!("/{}/zones/{}/records/{}", account_id, zone, record);

        self.client.get::<ZoneRecordEndpoint>(&*path, None)
    }

    /// Update a zone record
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    /// `record`: The record id
    /// `payload`: The `ZoneRecordUpdatePayload` with the information to create the zone record
    pub fn update_zone_record(&self, account_id: u64, zone: &str, record: u64, payload: ZoneRecordUpdatePayload) -> Result<DNSimpleResponse<ZoneRecord>, String> {
        let path = format!("/{}/zones/{}/records/{}", account_id, zone, record);

        self.client.patch::<ZoneRecordEndpoint>(&*path, serde_json::to_value(payload).unwrap())
    }

    /// Delete a zone record
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    /// `record`: The record id
    pub fn delete_zone_record(&self, account_id: u64, zone: &str, record: u64) -> DNSimpleEmptyResponse {
        let path = format!("/{}/zones/{}/records/{}", account_id, zone, record);

        self.client.delete(&*path)
    }

    /// Check zone record distribution
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    /// `record`: The record id
    pub fn check_zone_record_distribution(&self, account_id: u64, zone: &str, record: u64) -> Result<DNSimpleResponse<Distribution>, String> {
        let path = format!("/{}/zones/{}/records/{}/distribution", account_id, zone, record);

        self.client.get::<DistributionEndpoint>(&*path, None)
    }
}