use crate::dnsimple::DNSimpleEmptyResponse;
use crate::dnsimple::registrar::Registrar;

impl Registrar<'_> {
    /// Enable domain auto-renewal
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The domain name or id
    pub fn enable_domain_auto_renewal(&self, account_id: u64, domain: String) -> DNSimpleEmptyResponse {
        let path = format!("/{}/registrar/domains/{}/auto_renewal", account_id, domain);

        self.client.empty_put(&*path)
    }

    /// Disable domain auto-renewal
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The domain name or id
    pub fn disable_domain_auto_renewal(&self, account_id: u64, domain: String) -> DNSimpleEmptyResponse {
        let path = format!("/{}/registrar/domains/{}/auto_renewal", account_id, domain);

        self.client.delete(&*path)
    }
}
