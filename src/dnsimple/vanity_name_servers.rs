use crate::dnsimple::registrar_name_servers::VanityNameServer;
use crate::dnsimple::{Client, DNSimpleEmptyResponse, DNSimpleResponse, Endpoint};
use crate::errors::DNSimpleError;
use serde_json::Value;

struct VanityNameServersEndpoint;

impl Endpoint for VanityNameServersEndpoint {
    type Output = Vec<VanityNameServer>;
}

/// The Vanity Name Servers Service handles the vanity name servers of the DNSimple API.
///
/// See [API Documentation: vanity](https://developer.dnsimple.com/v2/vanity/)
pub struct VanityNameServers<'a> {
    pub client: &'a Client,
}

impl VanityNameServers<'_> {
    /// Enable vanity name servers
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `domain`: The domain name or id
    pub async fn enable_vanity_name_servers(
        &self,
        account_id: u64,
        domain: String,
    ) -> Result<DNSimpleResponse<Vec<VanityNameServer>>, DNSimpleError> {
        let path = format!("/{}/vanity/{}", account_id, domain);

        self.client
            .put::<VanityNameServersEndpoint>(&path, Value::Null)
            .await
    }

    /// Enable vanity name servers
    ///
    /// # Arguments
    /// `account_id`: The account id
    /// `domain`: The domain name or id
    pub async fn disable_vanity_name_servers(
        &self,
        account_id: u64,
        domain: String,
    ) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let path = format!("/{}/vanity/{}", account_id, domain);

        self.client.delete(&path).await
    }
}
