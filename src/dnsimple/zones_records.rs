use crate::dnsimple::zones::DistributionEndpoint;
use crate::dnsimple::zones::{ZoneDistribution, Zones};
use crate::dnsimple::{DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, RequestOptions};
use crate::errors::DNSimpleError;
use serde::{Deserialize, Serialize};

/// Represents a zone record in DNSimple
#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneRecord {
    /// The record ID in DNSimple.
    pub id: u64,
    /// The associated zone ID.
    pub zone_id: String,
    /// The ID of the parent record, if this record is dependent on another record.
    #[deprecated(
        note = "the value is always None, and the field will be removed in a future major version"
    )]
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
    #[serde(rename = "type")]
    pub record_type: String,
    /// The plain-text record content.
    pub content: String,
    /// The TTL value.
    pub ttl: Option<u64>,
    /// The priority value, if the type of record accepts a priority.
    pub priority: Option<u64>,
    /// The regions where the record is propagated. This is optional.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

/// Represents a zone record to create in a batch change
#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneRecordBatchCreate {
    /// The record name (without the domain name).
    pub name: String,
    /// The type of record, in uppercase.
    #[serde(rename = "type")]
    pub record_type: String,
    /// The plain-text record content.
    pub content: String,
    /// The TTL value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u64>,
    /// The priority value, if the type of record accepts a priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u64>,
    /// The regions where the record is propagated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

/// Represents a zone record to update in a batch change
#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneRecordBatchUpdate {
    /// The record ID in DNSimple.
    pub id: u64,
    /// The record name (without the domain name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The plain-text record content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The TTL value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u64>,
    /// The priority value, if the type of record accepts a priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u64>,
    /// The regions where the record is propagated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

/// Represents a zone record to delete in a batch change
#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneRecordBatchDelete {
    /// The record ID in DNSimple.
    pub id: u64,
}

/// Payload to create, update, and delete zone records in a batch change
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ZoneRecordsBatchChangePayload {
    /// The zone records to create.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates: Option<Vec<ZoneRecordBatchCreate>>,
    /// The zone records to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<ZoneRecordBatchUpdate>>,
    /// The zone records to delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletes: Option<Vec<ZoneRecordBatchDelete>>,
}

/// Represents the result of a zone records batch change
#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneRecordsBatchChange {
    /// The zone records that the batch change created.
    pub creates: Vec<ZoneRecord>,
    /// The zone records that the batch change updated.
    pub updates: Vec<ZoneRecord>,
    /// The zone records that the batch change deleted.
    pub deletes: Vec<ZoneRecordBatchDelete>,
}

struct ZoneRecordsEndpoint;

impl Endpoint for ZoneRecordsEndpoint {
    type Output = Vec<ZoneRecord>;
}

struct ZoneRecordEndpoint;

impl Endpoint for ZoneRecordEndpoint {
    type Output = ZoneRecord;
}

struct ZoneRecordsBatchChangeEndpoint;

impl Endpoint for ZoneRecordsBatchChangeEndpoint {
    type Output = ZoneRecordsBatchChange;
}

impl Zones<'_> {
    /// List zone records
    ///
    /// See [API Documentation](https://developer.dnsimple.com/v2/zones/records/#listZoneRecords)
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    pub async fn list_zone_records(
        &self,
        account_id: u64,
        zone: &str,
        options: Option<RequestOptions>,
    ) -> Result<DNSimpleResponse<Vec<ZoneRecord>>, DNSimpleError> {
        let path = format!("/{}/zones/{}/records", account_id, zone);

        self.client.get::<ZoneRecordsEndpoint>(&path, options).await
    }

    /// Create a zone record
    ///
    /// See [API Documentation](https://developer.dnsimple.com/v2/zones/records/#createZoneRecord)
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    /// `payload`: The `ZoneRecordPayload` with the information to create the zone record
    pub async fn create_zone_record(
        &self,
        account_id: u64,
        zone: &str,
        payload: ZoneRecordPayload,
    ) -> Result<DNSimpleResponse<ZoneRecord>, DNSimpleError> {
        let path = format!("/{}/zones/{}/records", account_id, zone);

        self.client.post::<ZoneRecordEndpoint>(&path, payload).await
    }

    /// Retrieve a zone record
    ///
    /// See [API Documentation](https://developer.dnsimple.com/v2/zones/records/#getZoneRecord)
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    /// `record`: The record id
    pub async fn get_zone_record(
        &self,
        account_id: u64,
        zone: &str,
        record: u64,
    ) -> Result<DNSimpleResponse<ZoneRecord>, DNSimpleError> {
        let path = format!("/{}/zones/{}/records/{}", account_id, zone, record);

        self.client.get::<ZoneRecordEndpoint>(&path, None).await
    }

    /// Update a zone record
    ///
    /// See [API Documentation](https://developer.dnsimple.com/v2/zones/records/#updateZoneRecord)
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    /// `record`: The record id
    /// `payload`: The `ZoneRecordUpdatePayload` with the information to create the zone record
    pub async fn update_zone_record(
        &self,
        account_id: u64,
        zone: &str,
        record: u64,
        payload: ZoneRecordUpdatePayload,
    ) -> Result<DNSimpleResponse<ZoneRecord>, DNSimpleError> {
        let path = format!("/{}/zones/{}/records/{}", account_id, zone, record);

        self.client
            .patch::<ZoneRecordEndpoint>(&path, payload)
            .await
    }

    /// Delete a zone record
    ///
    /// See [API Documentation](https://developer.dnsimple.com/v2/zones/records/#deleteZoneRecord)
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    /// `record`: The record id
    pub async fn delete_zone_record(
        &self,
        account_id: u64,
        zone: &str,
        record: u64,
    ) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let path = format!("/{}/zones/{}/records/{}", account_id, zone, record);

        self.client.delete(&path).await
    }

    /// Create, update, and delete zone records in a batch change
    ///
    /// The API applies all the changes atomically.
    ///
    /// See [API Documentation](https://developer.dnsimple.com/v2/zones/records/#batchChangeZoneRecords)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    /// use dnsimple::dnsimple::zones_records::{ZoneRecordBatchDelete, ZoneRecordsBatchChangePayload};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() {
    ///     let client = new_client(true, String::from("AUTH_TOKEN")).unwrap();
    ///     let payload = ZoneRecordsBatchChangePayload {
    ///         deletes: Some(vec![ZoneRecordBatchDelete { id: 1 }]),
    ///         ..Default::default()
    ///     };
    ///     let batch_change = client.zones().batch_change_zone_records(1234, "example.com", payload).await.unwrap().data.unwrap();
    /// }
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    /// `payload`: The `ZoneRecordsBatchChangePayload` with the zone records to create, update, and delete
    pub async fn batch_change_zone_records(
        &self,
        account_id: u64,
        zone: &str,
        payload: ZoneRecordsBatchChangePayload,
    ) -> Result<DNSimpleResponse<ZoneRecordsBatchChange>, DNSimpleError> {
        let path = format!("/{}/zones/{}/batch", account_id, zone);

        self.client
            .post::<ZoneRecordsBatchChangeEndpoint>(&path, payload)
            .await
    }

    /// Check zone record distribution
    ///
    /// See [API Documentation](https://developer.dnsimple.com/v2/zones/records/#checkZoneRecordDistribution)
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    /// `record`: The record id
    pub async fn check_zone_record_distribution(
        &self,
        account_id: u64,
        zone: &str,
        record: u64,
    ) -> Result<DNSimpleResponse<ZoneDistribution>, DNSimpleError> {
        let path = format!(
            "/{}/zones/{}/records/{}/distribution",
            account_id, zone, record
        );

        self.client.get::<DistributionEndpoint>(&path, None).await
    }
}
