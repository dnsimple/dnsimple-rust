use crate::dnsimple::domains::Domains;
use crate::dnsimple::{DNSimpleEmptyResponse, DNSimpleResponse, Endpoint, Filters, Paginate, Sort};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SignerRecord {
    pub id: u64,
    pub domain_id: u64,
    pub algorithm: String,
    pub digest: String,
    pub digest_type: String,
    pub keytag: String,
    pub public_key: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

struct ListSignerRecordsEndpoint;

impl Endpoint for ListSignerRecordsEndpoint {
    type Output = Vec<SignerRecord>;
}

#[derive(Debug, Serialize)]
pub struct SignerRecordPayload {
    pub algorithm: String,
    pub digest: String,
    pub digest_type: String,
    pub keytag: String,
    pub public_key: Option<String>,
}

struct SignerRecordEndpoint;

impl Endpoint for SignerRecordEndpoint {
    type Output = SignerRecord;
}

impl Domains<'_> {
    /// List delegation signer records for the domain in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want list the signer records from
    pub fn list_delegation_signer_records(&self, account_id: u64, domain: String, filters: Filters, sort: Sort, paginate: Paginate) -> Result<DNSimpleResponse<Vec<SignerRecord>>, String> {
        let path = format!("/{}/domains/{}/ds_records", account_id, domain);

        self.client.get::<ListSignerRecordsEndpoint>(&*path, filters, sort, paginate)
    }

    /// Creates a delegation signer record
    ///
    /// You only need to create a delegation signer record manually if your domain is registered
    /// with DNSimple but hosted with another DNS provider that is signing your zone.
    /// To enable DNSSEC on a domain that is hosted with DNSimple, use the DNSSEC enable endpoint.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want list the signer records from
    /// `payload`: The `SignerRecordPayload` with the data needed to create the delegation signer record
    pub fn create_delegation_signer_record(&self, account_id: u64, domain: String, payload: SignerRecordPayload) -> Result<DNSimpleResponse<SignerRecord>, String> {
        let path = format!("/{}/domains/{}/ds_records", account_id, domain);

        self.client.post::<SignerRecordEndpoint>(&*path, serde_json::to_value(payload).unwrap())
    }

    /// Get the delegation signer record under the domain for the account
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want list the signer records from
    pub fn get_delegation_signer_record(&self, account_id: u64, domain: String) -> Result<DNSimpleResponse<SignerRecord>, String> {
        let path = format!("/{}/domains/{}/ds_records", account_id, domain);
        
        self.client.get::<SignerRecordEndpoint>(&*path, Filters { filters: Default::default() }, Sort { sort_by: "".to_string() }, Paginate { per_page: 0, page: 0 })
    }

    /// Delete a Delegation Signer record
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The ID or name of the domain we want list the signer records from
    /// `ds_record_id`:	The delegation signer record id
    pub fn delete_delegation_signer_record(&self, account_id: u64, domain: String, delegation_signer_record_id: i32) -> DNSimpleEmptyResponse {
        let path = format!("/{}/domains/{}/ds_records/{}", account_id, domain, delegation_signer_record_id);

        self.client.delete(&*path)
    }
}

