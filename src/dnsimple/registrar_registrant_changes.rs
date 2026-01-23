use crate::dnsimple::registrar::Registrar;
use crate::dnsimple::tlds::TldExtendedAttribute;
use crate::dnsimple::{DNSimpleResponse, Endpoint, RequestOptions};
use crate::errors::DNSimpleError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the contact change data
#[derive(Debug, Deserialize)]
pub struct RegistrantChange {
    /// The contact change id in DNSimple
    pub id: u64,
    /// The associated account ID.
    pub account_id: u64,
    /// The associated contact ID.
    pub contact_id: u64,
    /// The associated domain ID.
    pub domain_id: u64,
    /// The registrant change state.
    pub state: String,
    /// The extended attributes.
    pub extended_attributes: Option<HashMap<String, String>>,
    /// True if the registrant change is a registry owner change.
    pub registry_owner_change: bool,
    /// When the Inter-Registrar Transfer lock (60 days) is going to be lifted.
    pub irt_lock_lifted_by: Option<String>,
    /// When the registrant change was created in DNSimple.
    pub created_at: String,
    /// When the registrant change was last updated in DNSimple.
    pub updated_at: String,
}

/// Represents the contact change check data
#[derive(Debug, Deserialize)]
pub struct RegistrantChangeCheck {
    /// The associated contact ID.
    pub contact_id: u64,
    /// The associated domain ID.
    pub domain_id: u64,
    /// The extended attributes.
    pub extended_attributes: Option<Vec<TldExtendedAttribute>>,
    /// True if the registrant change is a registry owner change.
    pub registry_owner_change: bool,
}

/// Payload used to check the requirements for a contact change
#[derive(Debug, Deserialize, Serialize)]
pub struct RegistrantChangePayload {
    /// The associated domain ID.
    pub domain_id: u64,
    /// The associated registrant (contact) ID.
    pub contact_id: u64,
    // The extended attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_attributes: Option<HashMap<String, String>>,
}

/// Payload used to check the requirements for a contact change
#[derive(Debug, Deserialize, Serialize)]
pub struct RegistrantChangeCheckPayload {
    /// The associated domain ID.
    pub domain_id: u64,
    /// The associated registrant (contact) ID.
    pub contact_id: u64,
}

struct RegistrantChangeEndpoint;

impl Endpoint for RegistrantChangeEndpoint {
    type Output = RegistrantChange;
}

struct DeleteRegistrantChangeEndpoint;

impl Endpoint for DeleteRegistrantChangeEndpoint {
    type Output = Option<RegistrantChange>;
}

struct RegistrantChangesEndpoint;

impl Endpoint for RegistrantChangesEndpoint {
    type Output = Vec<RegistrantChange>;
}

struct RegistrantChangeCheckEndpoint;

impl Endpoint for RegistrantChangeCheckEndpoint {
    type Output = RegistrantChangeCheck;
}

impl Registrar<'_> {
    /// Retrieve the domain contact change
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `registrant_change_id`: The contact change ID
    pub async fn get_registrant_change(
        &self,
        account_id: u64,
        registrant_change_id: u64,
    ) -> Result<DNSimpleResponse<RegistrantChange>, DNSimpleError> {
        let path = format!(
            "/{}/registrar/registrant_changes/{}",
            account_id, registrant_change_id
        );

        self.client.get::<RegistrantChangeEndpoint>(&path, None).await
    }

    /// Retrieves the requirements of a registrant change
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `payload`: The `RegistrantChangeCheckPayload` with the information needed to check the
    /// requirements for a registrant change
    pub async fn check_registrant_change(
        &self,
        account_id: u64,
        payload: RegistrantChangeCheckPayload,
    ) -> Result<DNSimpleResponse<RegistrantChangeCheck>, DNSimpleError> {
        let path = format!("/{}/registrar/registrant_changes/check", account_id);

        match serde_json::to_value(payload) {
            Ok(json) => self
                .client
                .post::<RegistrantChangeCheckEndpoint>(&path, json)
                .await,
            Err(_) => Err(DNSimpleError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }

    /// Start registrant change.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `payload`: The `RegistrantChangePayload` with the information needed to start a registrant change
    pub async fn create_registrant_change(
        &self,
        account_id: u64,
        payload: RegistrantChangePayload,
    ) -> Result<DNSimpleResponse<RegistrantChange>, DNSimpleError> {
        let path = format!("/{}/registrar/registrant_changes", account_id);

        match serde_json::to_value(payload) {
            Ok(json) => self.client.post::<RegistrantChangeEndpoint>(&path, json).await,
            Err(_) => Err(DNSimpleError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }

    /// List registrant changes in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `options`: The `RequestOptions`
    ///             - Filters: `domain_id`, `state`, `contact_id`
    ///             - Sorting: `id`
    pub async fn list_registrant_changes(
        &self,
        account_id: u64,
        options: Option<RequestOptions>,
    ) -> Result<DNSimpleResponse<Vec<RegistrantChange>>, DNSimpleError> {
        let path = format!("/{}/registrar/registrant_changes", account_id);

        self.client.get::<RegistrantChangesEndpoint>(&path, options).await
    }

    /// Cancel a registrant change.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `registrant_change_id`: The contact change ID
    pub async fn delete_registrant_change(
        &self,
        account_id: u64,
        registrant_change_id: u64,
    ) -> Result<DNSimpleResponse<Option<RegistrantChange>>, DNSimpleError> {
        let path = format!(
            "/{}/registrar/registrant_changes/{}",
            account_id, registrant_change_id
        );

        self.client
            .delete_with_response::<DeleteRegistrantChangeEndpoint>(&path)
            .await
    }
}
