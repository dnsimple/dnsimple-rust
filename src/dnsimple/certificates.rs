use crate::dnsimple::{Client, DNSimpleResponse, Endpoint, Filters, Paginate, Sort};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Certificate {
    pub id: u64,
    pub domain_id: u64,
    pub contact_id: u64,
    pub name: String,
    pub common_name: String,
    pub years: u32,
    pub csr: Option<String>,
    pub state: String,
    pub auto_renew: bool,
    pub alternate_names: Vec<String>,
    pub authority_identifier: String,
    pub created_at: String,
    pub updated_at: String,
    pub expires_at: Option<String>,
    pub expires_on: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CertificateDownload {
    pub server: String,
    pub root: Option<String>,
    pub chain: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CertificatePrivateKey {
    pub private_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LetsEncryptPurchase {
    pub id: u64,
    pub certificate_id: u64,
    pub state: String,
    pub auto_renew: bool,
    pub created_at: String,
    pub updated_at: String,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct LetsEncryptPurchaseRenewal {
    pub id: u64,
    pub old_certificate_id: u64,
    pub new_certificate_id: u64,
    pub state: String,
    pub auto_renew: bool,
    pub created_at: String,
    pub updated_at: String,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct LetsEncryptPurchasePayload {
    pub contact_id: u64,
    pub auto_renew: bool,
    pub name: String,
    pub alternate_names: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LetsEncryptPurchaseRenewalPayload {
    pub auto_renew: bool,
}

struct ListCertificatesEndpoint;

impl Endpoint for ListCertificatesEndpoint {
    type Output = Vec<Certificate>;
}

struct CertificateEndpoint;

impl Endpoint for CertificateEndpoint {
    type Output = Certificate;
}

struct CertificateDownloadEndpoint;

impl Endpoint for CertificateDownloadEndpoint {
    type Output = CertificateDownload;
}

struct CertificatePrivateKeyEndpoint;

impl Endpoint for CertificatePrivateKeyEndpoint {
    type Output = CertificatePrivateKey;
}

struct LetsEncryptPurchaseEndpoint;

impl Endpoint for LetsEncryptPurchaseEndpoint {
    type Output = LetsEncryptPurchase;
}

struct LetsEncryptPurchaseRenewalEndpoint;

impl Endpoint for LetsEncryptPurchaseRenewalEndpoint {
    type Output = LetsEncryptPurchaseRenewal;
}

pub struct Certificates<'a> {
    pub client: &'a Client
}

impl Certificates<'_> {
    /// List the certificates for a domain in the account.
    ///
    /// # Arguments
    /// `account_id`: The id of the account
    /// `domain`: The domain name or id
    pub fn list_certificates(&self, account_id: u64, domain: String, sort: Sort, paginate: Paginate) -> Result<DNSimpleResponse<Vec<Certificate>>, String> {
        let path = format!("/{}/domains/{}/certificates", account_id, domain);

        self.client.get::<ListCertificatesEndpoint>(&*path, Filters{ filters: Default::default() }, sort, paginate)
    }

    /// Get the details of a certificate
    ///
    /// # Arguments
    /// `account_id`: The id of the account
    /// `domain`: The domain name or id
    /// `certificate_id`: The certificate id
    pub fn get_certificate(&self, account_id: u64, domain: String, certificate_id: u64) -> Result<DNSimpleResponse<Certificate>, String> {
        let path = format!("/{}/domains/{}/certificates/{}", account_id, domain, certificate_id);

        self.client.get::<CertificateEndpoint>(&*path, Filters{ filters: Default::default() }, Sort{ sort_by: "".to_string() }, Paginate{ per_page: 0, page: 0 })
    }

    /// Download a certificate
    ///
    /// # Arguments
    /// `account_id`: The id of the account
    /// `domain`: The domain name or id
    /// `certificate_id`: The certificate id
    pub fn download_certificate(&self, account_id: u64, domain: String, certificate_id: u64) -> Result<DNSimpleResponse<CertificateDownload>, String> {
        let path = format!("/{}/domains/{}/certificates/{}/download", account_id, domain, certificate_id);

        self.client.get::<CertificateDownloadEndpoint>(&*path, Filters{ filters: Default::default() }, Sort{ sort_by: "".to_string() }, Paginate{ per_page: 0, page: 0 })
    }

    /// Get the PEM-encoded certificate private key
    ///
    /// # Arguments
    /// `account_id`: The id of the account
    /// `domain`: The domain name or id
    /// `certificate_id`: The certificate id
    pub fn get_certificate_private_key(&self, account_id: u64, domain: String, certificate_id: u64) -> Result<DNSimpleResponse<CertificatePrivateKey>, String> {
        let path = format!("/{}/domains/{}/certificates/{}/private_key", account_id, domain, certificate_id);

        self.client.get::<CertificatePrivateKeyEndpoint>(&*path, Filters{ filters: Default::default() }, Sort{ sort_by: "".to_string() }, Paginate{ per_page: 0, page: 0 })
    }

    /// Purchase a Let’s Encrypt certificate with DNSimple.
    ///
    /// # Arguments
    /// `account_id`: The id of the account
    /// `domain`: The domain name or id
    /// `payload`: The `LetsEncryptPurchasePayload` containing the information to purchase the certificate
    pub fn purchase_letsencrypt_certificate(&self, account_id: u64, domain: String, payload: LetsEncryptPurchasePayload) -> Result<DNSimpleResponse<LetsEncryptPurchase>, String> {
        let path = format!("/{}/domains/{}/certificates/letsencrypt", account_id, domain);

        self.client.post::<LetsEncryptPurchaseEndpoint>(&*path, serde_json::to_value(payload).unwrap())
    }

    /// Issue a Let’s Encrypt certificate for a domain in the account
    ///
    /// # Arguments
    /// `account_id`: The id of the account
    /// `domain`: The domain name or id
    /// `certificate_id`: The id of the certificate to be issued
    pub fn issue_letsencrypt_certificate(&self, account_id: u64, domain: String, certificate_id: u64) -> Result<DNSimpleResponse<Certificate>, String> {
        let path = format!("/{}/domains/{}/certificates/letsencrypt/{}/issue", account_id, domain, certificate_id);

        self.client.post::<CertificateEndpoint>(&*path, Value::Null)
    }

    /// Purchase a Let’s Encrypt certificate renewal
    ///
    /// # Arguments
    /// `account_id`: The id of the account
    /// `domain`: The domain name or id
    /// `payload`: The `LetsEncryptPurchaseRenewalPayload` containing the information to purchase the certificate
    pub fn purchase_letsencrypt_certificate_renewal(&self, account_id: u64, domain: String, certificate_id: u64, payload: LetsEncryptPurchaseRenewalPayload) -> Result<DNSimpleResponse<LetsEncryptPurchaseRenewal>, String> {
        let path = format!("/{}/domains/{}/certificates/letsencrypt/{}/renewals", account_id, domain, certificate_id);

        self.client.post::<LetsEncryptPurchaseRenewalEndpoint>(&*path, serde_json::to_value(payload).unwrap())
    }

    /// Issue a Let’s Encrypt certificate for a domain in the account
    ///
    /// # Arguments
    /// `account_id`: The id of the account
    /// `domain`: The domain name or id
    /// `certificate_id`: The id of the certificate to be issued
    /// `certificate_renewal_id`: The certificate renewal id
    pub fn issue_letsencrypt_certificate_renewal(&self, account_id: u64, domain: String, certificate_id: u64, certificate_renewal_id: u64) -> Result<DNSimpleResponse<Certificate>, String> {
        let path = format!("/{}/domains/{}/certificates/letsencrypt/{}/renewals/{}/issue", account_id, domain, certificate_id, certificate_renewal_id);

        self.client.post::<CertificateEndpoint>(&*path, Value::Null)
    }
}