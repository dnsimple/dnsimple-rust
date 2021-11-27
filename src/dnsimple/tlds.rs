use crate::dnsimple::{Client, DNSimpleResponse, Endpoint, Paginate, RequestOptions, Sort};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename(serialize = "tld_type", deserialize = "type"))]
pub struct Tld {
    pub tld: String,
    pub tld_type: u64,
    pub whois_privacy: bool,
    pub auto_renew_only: bool,
    pub idn: bool,
    pub minimum_registration: u64,
    pub registration_enabled: bool,
    pub renewal_enabled: bool,
    pub transfer_enabled: bool,
    pub dnssec_interface_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TldExtendedAttribute {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub options: Vec<ExtendedAttributeOption>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtendedAttributeOption {
    pub title: String,
    pub value: String,
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

pub struct Tlds<'a> {
    pub client: &'a Client
}

impl Tlds<'_> {
    /// Returns the list of TLDs supported for registration or transfer.
    pub fn list_tlds(&self, sort: Sort, paginate: Paginate) -> Result<DNSimpleResponse<Vec<Tld>>, String> {
        let path = "/tlds";

        self.client.get::<ListTldsEndpoint>(&*path,
                                            Option::from(
                                                RequestOptions{filters: None,
                                                    sort: Some(sort), paginate: Some(paginate)})) }

    /// Retrieves the details of a supported TLD.
    ///
    /// # Attributes
    ///
    /// `tld`: The TLD name
    pub fn get_tld(&self, tld: String) -> Result<DNSimpleResponse<Tld>, String> {
        let path = format!("/tlds/{}", tld);

        self.client.get::<TldEndpoint>(&*path, None)
    }

    /// Lists the TLD Extended Attributes
    ///
    /// # Attributes
    ///
    /// `tld`: The TLD name
    pub fn get_tld_extended_attributes(&self, tld: String) -> Result<DNSimpleResponse<Vec<TldExtendedAttribute>>, String> {
        let path = format!("/tlds/{}/extended_attributes", tld);

        self.client.get::<ListTldsExtendedAttributesEndpoint>(&*path, None)
    }
}