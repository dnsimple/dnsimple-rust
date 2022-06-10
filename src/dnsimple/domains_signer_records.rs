use crate::dnsimple::domains::Domains;
use crate::dnsimple::{DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, RequestOptions};
use crate::errors::DNSimpleError;
use serde::{Deserialize, Serialize};

/// Represents a delegation signer record
#[derive(Debug, Deserialize, Serialize)]
pub struct DelegationSignerRecord {
    /// The ID of the delegation signer record in DNSimple.
    pub id: u64,
    /// The associated domain ID.
    pub domain_id: u64,
    /// The signing algorithm used.
    pub algorithm: String,
    /// The digest value.
    pub digest: String,
    /// The digest type used.
    pub digest_type: String,
    /// The keytag for the associated DNSKEY.
    pub keytag: String,
    /// The public key that references the corresponding DNSKEY record.
    pub public_key: Option<String>,
    /// When the delegation signing record was created in DNSimple.
    pub created_at: String,
    /// When the delegation signing record was last updated in DNSimple.
    pub updated_at: String,
}

struct ListSignerRecordsEndpoint;

impl Endpoint for ListSignerRecordsEndpoint {
    type Output = Vec<DelegationSignerRecord>;
}

// Payload to create delegation signer records
#[derive(Debug, Serialize)]
pub struct DelegationSignerRecordPayload {
    /// The signing algorithm used.
    pub algorithm: String,
    /// The digest value.
    pub digest: String,
    /// The digest type used.
    pub digest_type: String,
    /// The keytag for the associated DNSKEY.
    pub keytag: String,
    /// The public key that references the corresponding DNSlKEY record.
    pub public_key: Option<String>,
}

struct SignerRecordEndpoint;

impl Endpoint for SignerRecordEndpoint {
    type Output = DelegationSignerRecord;
}

/// The domains signer records set of endpoints
///
/// See [API Documentation: domains/dnssec](https://developer.dnsimple.com/v2/domains/dnssec)
impl Domains<'_> {
    /// List delegation signer records for the domain in the account.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let signer_records = client.domains().list_delegation_signer_records(1234, "example.com", None).unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want list the signer records from
    /// `options` The `RequestOptions`
    ///           - Sort: `id`, `created_at`
    ///           - Pagination
    pub fn list_delegation_signer_records(
        &self,
        account_id: u64,
        domain: &str,
        options: Option<RequestOptions>,
    ) -> Result<DNSimpleResponse<Vec<DelegationSignerRecord>>, DNSimpleError> {
        let path = format!("/{}/domains/{}/ds_records", account_id, domain);

        self.client
            .get::<ListSignerRecordsEndpoint>(&*path, options)
    }

    /// Creates a delegation signer record
    ///
    /// You only need to create a delegation signer record manually if your domain is registered
    /// with DNSimple but hosted with another DNS provider that is signing your zone.
    /// To enable DNSSEC on a domain that is hosted with DNSimple, use the DNSSEC enable endpoint.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::domains_signer_records::DelegationSignerRecordPayload;
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let payload = DelegationSignerRecordPayload {
    ///     algorithm: String::from("13"),
    ///     digest: String::from("684a1f049d7d082b7f98691657da5a65764913df7f065f6f8c36edf62d66ca03"),
    ///     digest_type: String::from("2"),
    ///     keytag: String::from("2371"),
    ///     public_key: None,
    /// };
    /// let signer_record = client.domains().create_delegation_signer_record(1234, "example.com", payload).unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want list the signer records from
    /// `payload`: The `SignerRecordPayload` with the data needed to create the delegation signer record
    pub fn create_delegation_signer_record(
        &self,
        account_id: u64,
        domain: &str,
        payload: DelegationSignerRecordPayload,
    ) -> Result<DNSimpleResponse<DelegationSignerRecord>, DNSimpleError> {
        let path = format!("/{}/domains/{}/ds_records", account_id, domain);

        match serde_json::to_value(payload) {
            Ok(json) => self.client.post::<SignerRecordEndpoint>(&*path, json),
            Err(_) => Err(DNSimpleError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }

    /// Get the delegation signer record under the domain for the account
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let signer_records = client.domains().get_delegation_signer_record(1234, "example.com").unwrap().data.unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want list the signer records from
    pub fn get_delegation_signer_record(
        &self,
        account_id: u64,
        domain: &str,
    ) -> Result<DNSimpleResponse<DelegationSignerRecord>, DNSimpleError> {
        let path = format!("/{}/domains/{}/ds_records", account_id, domain);

        self.client.get::<SignerRecordEndpoint>(&*path, None)
    }

    /// Delete a Delegation Signer record
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let response = client.domains().delete_delegation_signer_record(1234, "example.com", 42);
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want list the signer records from
    /// `ds_record_id`: The delegation signer record id
    pub fn delete_delegation_signer_record(
        &self,
        account_id: u64,
        domain: &str,
        delegation_signer_record_id: i32,
    ) -> Result<DNSimpleEmptyResponse, DNSimpleError> {
        let path = format!(
            "/{}/domains/{}/ds_records/{}",
            account_id, domain, delegation_signer_record_id
        );

        self.client.delete(&*path)
    }
}
