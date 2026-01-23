use crate::dnsimple::registrar::Registrar;
use crate::dnsimple::{DNSimpleEmptyResponse, DNSimpleResponse, Endpoint};
use crate::errors::DNSimpleError;
use serde::Deserialize;
use serde_json::Value;

struct DomainDelegationEndpoint;

impl Endpoint for DomainDelegationEndpoint {
    type Output = Vec<String>;
}

/// Represents a vanity name server
#[derive(Debug, Deserialize)]
pub struct VanityNameServer {
    /// The vanity name server ID in DNSimple.
    pub id: u64,
    /// The vanity name server name.
    pub name: String,
    /// The vanity name server IPv4.
    pub ipv4: String,
    /// The vanity name server IPv6.
    pub ipv6: String,
    /// When the vanity name server was created in DNSimple.
    pub created_at: String,
    /// When the vanity name server was last updated in DNSimple.
    pub updated_at: String,
}

struct DomainDelegationVanityEndpoint;

impl Endpoint for DomainDelegationVanityEndpoint {
    type Output = Vec<VanityNameServer>;
}

impl Registrar<'_> {
    /// List name servers for the domain in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The domain name or id
    pub async fn get_domain_delegation(
        &self,
        account_id: u64,
        domain: String,
    ) -> Result<DNSimpleResponse<Vec<String>>, DNSimpleError> {
        let path = format!("/{}/registrar/domains/{}/delegation", account_id, domain);

        self.client.get::<DomainDelegationEndpoint>(&path, None).await
    }

    /// Change domain name servers
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The domain name or id
    /// `server_names`: A list of name server names as strings
    pub async fn change_domain_delegation(
        &self,
        account_id: u64,
        domain: String,
        server_names: Vec<&str>,
    ) -> Result<DNSimpleResponse<Vec<String>>, DNSimpleError> {
        let path = format!("/{}/registrar/domains/{}/delegation", account_id, domain);

        self.client
            .put::<DomainDelegationEndpoint>(&path, Value::from(server_names))
            .await
    }

    /// Delegate to vanity name servers
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The domain name or id
    /// `server_names`: A list of name server names as strings
    pub async fn change_domain_delegation_to_vanity(
        &self,
        account_id: u64,
        domain: String,
        server_names: Vec<&str>,
    ) -> Result<DNSimpleResponse<Vec<VanityNameServer>>, DNSimpleError> {
        let path = format!(
            "/{}/registrar/domains/{}/delegation/vanity",
            account_id, domain
        );

        self.client
            .put::<DomainDelegationVanityEndpoint>(&path, Value::from(server_names))
            .await
    }

    /// De-delegate from vanity name servers
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The domain name or id
    pub async fn change_domain_delegation_from_vanity(
        &self,
        account_id: u64,
        domain: String,
    ) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let path = format!(
            "/{}/registrar/domains/{}/delegation/vanity",
            account_id, domain
        );

        self.client.delete(&path).await
    }
}
