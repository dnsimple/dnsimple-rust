use crate::dnsimple::{Client, DNSimpleResponse, Endpoint, RequestOptions};
use crate::errors::DNSimpleError;
use serde::{Deserialize, Serialize};

/// Represents a TLD in DNSimple
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename(serialize = "tld_type", deserialize = "type"))]
pub struct Tld {
    /// The TLD in DNSimple.
    pub tld: String,
    /// The TLD type.
    pub tld_type: u64,
    /// True if Whois Privacy Protection is available.
    pub whois_privacy: bool,
    ///  True if TLD requires use of auto-renewal for renewals.
    pub auto_renew_only: bool,
    /// True if IDN is available.
    pub idn: bool,
    /// The minimum registration period, in years.
    pub minimum_registration: u64,
    /// True if DNSimple supports registrations for this TLD.
    pub registration_enabled: bool,
    /// True if DNSimple supports renewals for this TLD.
    pub renewal_enabled: bool,
    /// True if DNSimple supports inbound transfers for this TLD.
    pub transfer_enabled: bool,
    /// Type of data interface required for DNSSEC for this TLD.
    pub dnssec_interface_type: Option<String>,
}

/// Represents an extended Attribute
#[derive(Debug, Deserialize, Serialize)]
pub struct TldExtendedAttribute {
    /// The extended attribute name
    pub name: String,
    /// A description of the extended attribute
    pub description: String,
    /// Boolean indicating if the extended attribute is required
    pub required: bool,
    /// The Vec of options with possible values for the extended attribute
    pub options: Vec<ExtendedAttributeOption>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtendedAttributeOption {
    /// he option name
    pub title: String,
    /// The option value
    pub value: String,
    /// A long description of the option
    pub description: String,
}

struct ListTldsEndpoint;

impl Endpoint for ListTldsEndpoint {
    type Output = Vec<Tld>;
}

struct TldEndpoint;

impl Endpoint for TldEndpoint {
    type Output = Tld;
}

struct ListTldsExtendedAttributesEndpoint;

impl Endpoint for ListTldsExtendedAttributesEndpoint {
    type Output = Vec<TldExtendedAttribute>;
}

/// The Tlds Service handles the tlds of the DNSimple API.
///
/// See [API Documentation: tlds](https://developer.dnsimple.com/v2/tlds/)
pub struct Tlds<'a> {
    pub client: &'a Client,
}

impl Tlds<'_> {
    /// Returns the list of TLDs supported for registration or transfer.
    pub async fn list_tlds(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<DNSimpleResponse<Vec<Tld>>, DNSimpleError> {
        let path = "/tlds";

        self.client.get::<ListTldsEndpoint>(path, options).await
    }

    /// Retrieves the details of a supported TLD.
    ///
    /// # Attributes
    ///
    /// `tld`: The TLD name
    pub async fn get_tld(&self, tld: String) -> Result<DNSimpleResponse<Tld>, DNSimpleError> {
        let path = format!("/tlds/{}", tld);

        self.client.get::<TldEndpoint>(&path, None).await
    }

    /// Lists the TLD Extended Attributes
    ///
    /// # Attributes
    ///
    /// `tld`: The TLD name
    pub async fn get_tld_extended_attributes(
        &self,
        tld: String,
    ) -> Result<DNSimpleResponse<Vec<TldExtendedAttribute>>, DNSimpleError> {
        let path = format!("/tlds/{}/extended_attributes", tld);

        self.client
            .get::<ListTldsExtendedAttributesEndpoint>(&path, None)
            .await
    }
}
