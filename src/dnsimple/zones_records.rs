use crate::dnsimple::zones::DistributionEndpoint;
use crate::dnsimple::zones::{ZoneDistribution, Zones};
use crate::dnsimple::{DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, RequestOptions};
use serde::{Deserialize, Serialize};

/// Represents a zone record in DNSimple
#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneRecord {
    /// The record ID in DNSimple.
    pub id: u64,
    /// The associated zone ID.
    pub zone_id: String,
    /// The ID of the parent record, if this record is dependent on another record.
    pub parent_id: Option<String>,
    /// The record name (without the domain name).
    pub name: String,
    /// The plain-text record content.
    pub content: String,
    /// The TTL value.
    pub ttl: u64,
    /// The priority value, if the type of record accepts a priority.
    pub priority: Option<u64>,
    /// The type of record, in uppercase.
    #[serde(rename = "type")]
    pub record_type: String,
    /// The regions where the record is propagated. This is optional.
    pub regions: Option<Vec<String>>,
    /// True if this is a system record created by DNSimple. System records are read-only.
    pub system_record: bool,
    /// When the record was created in DNSimple.
    pub created_at: String,
    /// When the record was last updated in DNSimple.
    pub updated_at: String,
}

/// Represents the payload to be send to create a zone record
#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneRecordPayload {
    /// The record name (without the domain name).
    pub name: String,
    /// The type of record, in uppercase.
    pub record_type: String,
    /// The plain-text record content.
    pub content: String,
    /// The TTL value.
    pub ttl: Option<u64>,
    /// The priority value, if the type of record accepts a priority.
    pub priority: Option<u64>,
    /// The regions where the record is propagated. This is optional.
    pub regions: Option<Vec<String>>,
}

/// Represents the payload to be send to update a zone record
#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneRecordUpdatePayload {
    /// The record name (without the domain name).
    pub name: Option<String>,
    /// The plain-text record content.
    pub content: Option<String>,
    /// The TTL value.
    pub ttl: Option<u64>,
    /// The priority value, if the type of record accepts a priority.
    pub priority: Option<u64>,
    /// The regions where the record is propagated. This is optional.
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
    /// List zone records
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    pub fn list_zone_records(
        &self,
        account_id: u64,
        zone: &str,
        options: Option<RequestOptions>,
    ) -> Result<DNSimpleResponse<Vec<ZoneRecord>>, String> {
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
    pub fn create_zone_record(
        &self,
        account_id: u64,
        zone: &str,
        payload: ZoneRecordPayload,
    ) -> Result<DNSimpleResponse<ZoneRecord>, String> {
        let path = format!("/{}/zones/{}/records", account_id, zone);

        self.client
            .post::<ZoneRecordEndpoint>(&*path, serde_json::to_value(payload).unwrap())
    }

    /// Retrieve a zone record
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    /// `record`: The record id
    pub fn get_zone_record(
        &self,
        account_id: u64,
        zone: &str,
        record: u64,
    ) -> Result<DNSimpleResponse<ZoneRecord>, String> {
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
    pub fn update_zone_record(
        &self,
        account_id: u64,
        zone: &str,
        record: u64,
        payload: ZoneRecordUpdatePayload,
    ) -> Result<DNSimpleResponse<ZoneRecord>, String> {
        let path = format!("/{}/zones/{}/records/{}", account_id, zone, record);

        self.client
            .patch::<ZoneRecordEndpoint>(&*path, serde_json::to_value(payload).unwrap())
    }

    /// Delete a zone record
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    /// `record`: The record id
    pub fn delete_zone_record(
        &self,
        account_id: u64,
        zone: &str,
        record: u64,
    ) -> DNSimpleEmptyResponse {
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
    pub fn check_zone_record_distribution(
        &self,
        account_id: u64,
        zone: &str,
        record: u64,
    ) -> Result<DNSimpleResponse<ZoneDistribution>, String> {
        let path = format!(
            "/{}/zones/{}/records/{}/distribution",
            account_id, zone, record
        );

        self.client.get::<DistributionEndpoint>(&*path, None)
    }
}
