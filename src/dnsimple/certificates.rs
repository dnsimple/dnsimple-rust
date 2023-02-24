use crate::dnsimple::{Client, DNSimpleResponse, Endpoint, RequestOptions};
use crate::errors::DNSimpleError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a certificate
#[derive(Debug, Deserialize, Serialize)]
pub struct Certificate {
    /// The certificate ID in DNSimple.
    pub id: u64,
    /// The associated domain ID.
    pub domain_id: u64,
    /// The associated contact ID.
    #[deprecated]
    pub contact_id: u64,
    /// The certificate name.
    pub name: String,
    /// The certificate common name.
    pub common_name: String,
    /// The years the certificate will last.
    pub years: u32,
    /// The certificate CSR.
    pub csr: Option<String>,
    /// The certificate state.
    pub state: String,
    /// True if the certificate is set to auto-renew on expiration.
    pub auto_renew: bool,
    /// The certificate alternate names.
    pub alternate_names: Vec<String>,
    /// The Certificate Authority (CA) that issued the certificate.
    pub authority_identifier: String,
    /// When the certificate was created in DNSimple.
    pub created_at: String,
    /// When the certificate was last updated in DNSimple.
    pub updated_at: String,
    /// The timestamp when the certificate will expire.
    pub expires_at: Option<String>,
    /// The day when the certificate will expire.
    pub expires_on: Option<String>,
}

/// Represents the certificate bundle when downloading a certificate
#[derive(Debug, Deserialize, Serialize)]
pub struct CertificateBundle {
    /// The server certificate
    pub server: String,
    /// The root certificate
    pub root: Option<String>,
    /// Intermediate certificates
    pub chain: Vec<String>,
}

/// Represents the private key of a certificate
#[derive(Debug, Deserialize, Serialize)]
pub struct CertificatePrivateKey {
    /// The certificate private key
    pub private_key: String,
}

/// The result of a Let's Encrypt Certificate purchase
#[derive(Debug, Deserialize, Serialize)]
pub struct LetsEncryptPurchase {
    /// The id of the purchase
    pub id: u64,
    /// The id of the certificate
    pub certificate_id: u64,
    /// The state of the purchase
    pub state: String,
    /// True if the certificate will auto renew
    pub auto_renew: bool,
    /// When the purchase was created
    pub created_at: String,
    /// When the purchase was last updated
    pub updated_at: String,
}

/// A renewal for a Let's Encrypt Purchase
#[derive(Debug, Deserialize, Serialize)]
pub struct LetsEncryptPurchaseRenewal {
    /// The id of the renewal
    pub id: u64,
    /// The previous id of the certificate
    pub old_certificate_id: u64,
    /// The id of the certificate after the renewal
    pub new_certificate_id: u64,
    /// The state of the renewal
    pub state: String,
    /// True if the certificate will auto renew
    pub auto_renew: bool,
    /// When the renewal was created
    pub created_at: String,
    /// When the renewal was last updated
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LetsEncryptSignatureAlgorithm {
  ECDSA,
  RSA,
}

/// The payload for purchasing a Let's Encrypt Certificate
#[derive(Debug, Deserialize, Serialize)]
pub struct LetsEncryptPurchasePayload {
    /// Set to true to enable the auto-renewal of the certificate.
    pub auto_renew: bool,
    /// The certificate name.
    pub name: String,
    /// The certificate alternate names (i.e. ["docs.example.com", "status.example.com"])
    pub alternate_names: Vec<String>,
    /// Signature algorithm to be used.
    pub signature_algorithm: LetsEncryptSignatureAlgorithm,
}

/// The payload for renewing a Let's Encrypt Certificate
#[derive(Debug, Deserialize, Serialize)]
pub struct LetsEncryptPurchaseRenewalPayload {
    /// Set to true to enable auto-renewal of the certificate
    pub auto_renew: bool,
    /// Signature algorithm to be used.
    pub signature_algorithm: LetsEncryptSignatureAlgorithm,
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
    type Output = CertificateBundle;
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

/// The Certificates Service handles the certificates endpoint of the DNSimple API.
///
/// See [API Documentation: certificates](https://developer.dnsimple.com/v2/certificates/)
pub struct Certificates<'a> {
    pub client: &'a Client,
}

impl Certificates<'_> {
    /// List the certificates for a domain in the account.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::{Client, new_client};
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let certificates = client.certificates().list_certificates(1010, "example.com", None).unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The id of the account
    /// `domain`: The domain name or id
    /// `options`: The `RequestOptions`.
    ///            - Sorting: `id`, `common_name`, `expiration`
    pub fn list_certificates(
        &self,
        account_id: u64,
        domain: &str,
        options: Option<RequestOptions>,
    ) -> Result<DNSimpleResponse<Vec<Certificate>>, DNSimpleError> {
        let path = format!("/{}/domains/{}/certificates", account_id, domain);

        self.client.get::<ListCertificatesEndpoint>(&path, options)
    }

    /// Get the details of a certificate
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::{Client, new_client};
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let certificate = client.certificates().get_certificate(1010, "example.com", 42).unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The id of the account
    /// `domain`: The domain name or id
    /// `certificate_id`: The certificate id
    pub fn get_certificate(
        &self,
        account_id: u64,
        domain: &str,
        certificate_id: u64,
    ) -> Result<DNSimpleResponse<Certificate>, DNSimpleError> {
        let path = format!(
            "/{}/domains/{}/certificates/{}",
            account_id, domain, certificate_id
        );

        self.client.get::<CertificateEndpoint>(&path, None)
    }

    /// Download a certificate
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::{Client, new_client};
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let certificate = client.certificates().download_certificate(1010, "example.com", 42).unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The id of the account
    /// `domain`: The domain name or id
    /// `certificate_id`: The certificate id
    pub fn download_certificate(
        &self,
        account_id: u64,
        domain: &str,
        certificate_id: u64,
    ) -> Result<DNSimpleResponse<CertificateBundle>, DNSimpleError> {
        let path = format!(
            "/{}/domains/{}/certificates/{}/download",
            account_id, domain, certificate_id
        );

        self.client.get::<CertificateDownloadEndpoint>(&path, None)
    }

    /// Get the PEM-encoded certificate private key
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::{Client, new_client};
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let private_key = client.certificates().get_certificate_private_key(1010, "example.com", 42).unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The id of the account
    /// `domain`: The domain name or id
    /// `certificate_id`: The certificate id
    pub fn get_certificate_private_key(
        &self,
        account_id: u64,
        domain: &str,
        certificate_id: u64,
    ) -> Result<DNSimpleResponse<CertificatePrivateKey>, DNSimpleError> {
        let path = format!(
            "/{}/domains/{}/certificates/{}/private_key",
            account_id, domain, certificate_id
        );

        self.client
            .get::<CertificatePrivateKeyEndpoint>(&path, None)
    }

    /// Purchase a Let’s Encrypt certificate with DNSimple.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::{Client, new_client};
    /// use dnsimple::dnsimple::certificates::LetsEncryptPurchasePayload;
    /// use dnsimple::dnsimple::certificates::LetsEncryptSignatureAlgorithm;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let payload = LetsEncryptPurchasePayload {
    ///     auto_renew: true,
    ///     name: String::from("secret"),
    ///     alternate_names: vec![],
    ///     signature_algorithm: LetsEncryptSignatureAlgorithm::ECDSA,
    /// };
    /// let purchase = client.certificates().purchase_letsencrypt_certificate(1010, "example.com", payload).unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The id of the account
    /// `domain`: The domain name or id
    /// `payload`: The `LetsEncryptPurchasePayload` containing the information to purchase the certificate
    pub fn purchase_letsencrypt_certificate(
        &self,
        account_id: u64,
        domain: &str,
        payload: LetsEncryptPurchasePayload,
    ) -> Result<DNSimpleResponse<LetsEncryptPurchase>, DNSimpleError> {
        let path = format!(
            "/{}/domains/{}/certificates/letsencrypt",
            account_id, domain
        );

        match serde_json::to_value(payload) {
            Ok(json) => self.client.post::<LetsEncryptPurchaseEndpoint>(&path, json),
            Err(_) => Err(DNSimpleError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }

    /// Issue a Let’s Encrypt certificate for a domain in the account
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::{Client, new_client};
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let issued = client.certificates().issue_letsencrypt_certificate(1010, "example.com", 42).unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The id of the account
    /// `domain`: The domain name or id
    /// `certificate_id`: The id of the certificate to be issued
    pub fn issue_letsencrypt_certificate(
        &self,
        account_id: u64,
        domain: &str,
        certificate_id: u64,
    ) -> Result<DNSimpleResponse<Certificate>, DNSimpleError> {
        let path = format!(
            "/{}/domains/{}/certificates/letsencrypt/{}/issue",
            account_id, domain, certificate_id
        );

        self.client.post::<CertificateEndpoint>(&path, Value::Null)
    }

    /// Purchase a Let’s Encrypt certificate renewal
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::{Client, new_client};
    /// use dnsimple::dnsimple::certificates::LetsEncryptPurchaseRenewalPayload;
    /// use dnsimple::dnsimple::certificates::LetsEncryptSignatureAlgorithm;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let payload = LetsEncryptPurchaseRenewalPayload {
    ///     auto_renew: false,
    ///     signature_algorithm: LetsEncryptSignatureAlgorithm::ECDSA,
    /// };
    /// let issued = client.certificates().purchase_letsencrypt_certificate_renewal(1010, "example.com", 42, payload).unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    /// `account_id`: The id of the account
    /// `domain`: The domain name or id
    /// `payload`: The `LetsEncryptPurchaseRenewalPayload` containing the information to purchase the certificate
    pub fn purchase_letsencrypt_certificate_renewal(
        &self,
        account_id: u64,
        domain: &str,
        certificate_id: u64,
        payload: LetsEncryptPurchaseRenewalPayload,
    ) -> Result<DNSimpleResponse<LetsEncryptPurchaseRenewal>, DNSimpleError> {
        let path = format!(
            "/{}/domains/{}/certificates/letsencrypt/{}/renewals",
            account_id, domain, certificate_id
        );

        match serde_json::to_value(payload) {
            Ok(json) => self
                .client
                .post::<LetsEncryptPurchaseRenewalEndpoint>(&path, json),
            Err(_) => Err(DNSimpleError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }

    /// Issue a Let’s Encrypt certificate for a domain in the account
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::{Client, new_client};
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let issued = client.certificates().issue_letsencrypt_certificate_renewal(1010, "example.com", 41, 42).unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    /// `account_id`: The id of the account
    /// `domain`: The domain name or id
    /// `certificate_id`: The id of the certificate to be issued
    /// `certificate_renewal_id`: The certificate renewal id
    pub fn issue_letsencrypt_certificate_renewal(
        &self,
        account_id: u64,
        domain: &str,
        certificate_id: u64,
        certificate_renewal_id: u64,
    ) -> Result<DNSimpleResponse<Certificate>, DNSimpleError> {
        let path = format!(
            "/{}/domains/{}/certificates/letsencrypt/{}/renewals/{}/issue",
            account_id, domain, certificate_id, certificate_renewal_id
        );

        self.client.post::<CertificateEndpoint>(&path, Value::Null)
    }
}
