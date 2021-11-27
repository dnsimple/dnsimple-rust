use crate::dnsimple::{Client, DNSimpleResponse, Endpoint, RequestOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Zone {
    pub id: u64,
    pub account_id: u64,
    pub name: String,
    pub reverse: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneFile  {
    pub zone: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneDistribution  {
    pub distributed: bool,
}

struct ListZonesEndpoint;

impl Endpoint for ListZonesEndpoint {
    type Output = Vec<Zone>;
}

struct ZoneEndpoint;

impl Endpoint for ZoneEndpoint {
    type Output = Zone;
}

struct ZoneFileEndpoint;

impl Endpoint for ZoneFileEndpoint {
    type Output = ZoneFile;
}

struct ZoneDistributionEndpoint;

impl Endpoint for ZoneDistributionEndpoint {
    type Output = ZoneDistribution;
}

pub struct Zones<'a> {
    pub client: &'a Client
}

impl Zones<'_> {

    /// Lists the zones in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    pub fn list_zones(&self, account_id: u64, options: Option<RequestOptions>) -> Result<DNSimpleResponse<Vec<Zone>>, String> {
        let path = format!("/{}/zones", account_id);

        self.client.get::<ListZonesEndpoint>(&*path, options)
    }

    /// Retrieve a zone
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    pub fn get_zone(&self, account_id: u64, zone: &str) -> Result<DNSimpleResponse<Zone>, String> {
        let path = format!("/{}/zones/{}", account_id, zone);

        self.client.get::<ZoneEndpoint>(&*path, None)
    }

    /// Download a zone file
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    pub fn get_zone_file(&self, account_id: u64, zone: &str) -> Result<DNSimpleResponse<ZoneFile>, String> {
        let path = format!("/{}/zones/{}/file", account_id, zone);

        self.client.get::<ZoneFileEndpoint>(&*path, None)
    }

    /// Check zone distribution
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `zone`: The zone name
    pub fn check_zone_distribution(&self, account_id: u64, zone: &str) -> Result<DNSimpleResponse<ZoneDistribution>, String> {
        let path = format!("/{}/zones/{}/distribution", account_id, zone);

        self.client.get::<ZoneDistributionEndpoint>(&*path, None)
    }
}