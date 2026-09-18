use crate::dnsimple::registrar::Registrar;
use crate::dnsimple::{DNSimpleResponse, Endpoint};
use crate::errors::DNSimpleError;
use serde::{Deserialize, Serialize};

/// Represents Transfer Lock status for a domain
#[derive(Debug, Deserialize, Serialize)]
pub struct TransferLock {
    /// True if domain transfer is locked, otherwise false
    pub enabled: bool,
}

struct DomainTransferLockEndpoint;

impl Endpoint for DomainTransferLockEndpoint {
    type Output = TransferLock;
}

impl Registrar<'_> {
    /// Enable domain transfer lock
    ///
    /// See [API Documentation](https://developer.dnsimple.com/v2/registrar/transfer-lock/#enableDomainTransferLock)
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The domain name or id
    pub async fn enable_domain_transfer_lock(
        &self,
        account_id: u64,
        domain: String,
    ) -> Result<DNSimpleResponse<TransferLock>, DNSimpleError> {
        let path = format!("/{}/registrar/domains/{}/transfer_lock", account_id, domain);

        self.client
            .post_without_payload::<DomainTransferLockEndpoint>(&path)
            .await
    }

    /// Disable domain transfer lock
    ///
    /// See [API Documentation](https://developer.dnsimple.com/v2/registrar/transfer-lock/#disableDomainTransferLock)
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The domain name or id
    pub async fn disable_domain_transfer_lock(
        &self,
        account_id: u64,
        domain: String,
    ) -> Result<DNSimpleResponse<TransferLock>, DNSimpleError> {
        let path = format!("/{}/registrar/domains/{}/transfer_lock", account_id, domain);

        self.client
            .delete_with_response::<DomainTransferLockEndpoint>(&path)
            .await
    }

    /// Get domain transfer lock status
    ///
    /// See [API Documentation](https://developer.dnsimple.com/v2/registrar/transfer-lock/#getDomainTransferLock)
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The domain name or id
    pub async fn get_domain_transfer_lock(
        &self,
        account_id: u64,
        domain: String,
    ) -> Result<DNSimpleResponse<TransferLock>, DNSimpleError> {
        let path = format!("/{}/registrar/domains/{}/transfer_lock", account_id, domain);

        self.client
            .get::<DomainTransferLockEndpoint>(&path, None)
            .await
    }
}
