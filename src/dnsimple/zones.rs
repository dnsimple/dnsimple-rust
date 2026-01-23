use crate::dnsimple::{Client, DNSimpleResponse, Endpoint, RequestOptions};
use crate::errors::DNSimpleError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a zone in DNSimple
#[derive(Debug, Deserialize, Serialize)]
pub struct Zone {
    /// The zone ID in DNSimple.
    pub id: u64,
    ///  The associated account ID.
    pub account_id: u64,
    /// The zone name.
    pub name: String,
    /// True if the zone is a reverse zone.
    pub reverse: bool,
    /// True if the zone is a secondary zone.
    pub secondary: bool,
    /// Last time the zone was transferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transferred_at: Option<String>,
    /// True if the zone is active.
    pub active: bool,
    ///  When the zone was created in DNSimple.
    pub created_at: String,
    ///  When the zone was created in DNSimple.
    pub updated_at: String,
}

/// Represents a zone file in DNSimple
#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneFile {
    /// The zone file contents.
    pub zone: String,
}

/// Represents a Zone Distribution in DNSimple
#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneDistribution {
    /// true if the zone is properly distributed across all DNSimple name servers.
    pub distributed: bool,
}

struct ListZonesEndpoint;

impl Endpoint for ListZonesEndpoint {
    type Output = Vec<Zone>;
}

struct ActivateDnsEndpoint;

impl Endpoint for ActivateDnsEndpoint {
    type Output = Zone;
}

struct DeactivateDnsEndpoint;

impl Endpoint for DeactivateDnsEndpoint {
    type Output = Zone;
}

struct ZoneEndpoint;

impl Endpoint for ZoneEndpoint {
    type Output = Zone;
}

struct ZoneFileEndpoint;

impl Endpoint for ZoneFileEndpoint {
    type Output = ZoneFile;
}

pub(crate) struct DistributionEndpoint;

impl Endpoint for DistributionEndpoint {
    type Output = ZoneDistribution;
}

/// The Zones Service handles the zone distribution of the DNSimple API.
///
/// See [API Documentation: zones](https://developer.dnsimple.com/v2/zones/)
pub struct Zones<'a> {
    pub client: &'a Client,
}

impl Zones<'_> {
    /// Activates DNS resolution for the zone in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone_name`: The zone name
    pub fn activate_dns(
        &self,
        account_id: u64,
        zone_name: &str,
    ) -> Result<DNSimpleResponse<Zone>, DNSimpleError> {
        let path = format!("/{}/zones/{}/activation", account_id, zone_name);

        self.client.put::<ActivateDnsEndpoint>(&path, Value::Null)
    }

    /// Deactivates DNS resolution for the zone in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone_name`: The zone name
    pub fn deactivate_dns(
        &self,
        account_id: u64,
        zone_name: &str,
    ) -> Result<DNSimpleResponse<Zone>, DNSimpleError> {
        let path = format!("/{}/zones/{}/activation", account_id, zone_name);

        self.client
            .delete_with_response::<DeactivateDnsEndpoint>(&path)
    }

    /// Lists the zones in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    pub fn list_zones(
        &self,
        account_id: u64,
        options: Option<RequestOptions>,
    ) -> Result<DNSimpleResponse<Vec<Zone>>, DNSimpleError> {
        let path = format!("/{}/zones", account_id);

        self.client.get::<ListZonesEndpoint>(&path, options)
    }

    /// Retrieve a zone
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    pub fn get_zone(
        &self,
        account_id: u64,
        zone: &str,
    ) -> Result<DNSimpleResponse<Zone>, DNSimpleError> {
        let path = format!("/{}/zones/{}", account_id, zone);

        self.client.get::<ZoneEndpoint>(&path, None)
    }

    /// Download a zone file
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    pub fn get_zone_file(
        &self,
        account_id: u64,
        zone: &str,
    ) -> Result<DNSimpleResponse<ZoneFile>, DNSimpleError> {
        let path = format!("/{}/zones/{}/file", account_id, zone);

        self.client.get::<ZoneFileEndpoint>(&path, None)
    }

    /// Check zone distribution
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    pub fn check_zone_distribution(
        &self,
        account_id: u64,
        zone: &str,
    ) -> Result<DNSimpleResponse<ZoneDistribution>, DNSimpleError> {
        let path = format!("/{}/zones/{}/distribution", account_id, zone);

        self.client.get::<DistributionEndpoint>(&path, None)
    }
}
