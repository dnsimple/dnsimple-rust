use crate::dnsimple::domains::Domains;
use crate::dnsimple::{DNSimpleResponse, Endpoint, Filters, RequestOptions};
use crate::errors::DNSimpleError;
use serde::{Deserialize, Serialize};

/// Represents the result of a domain research
#[derive(Debug, Deserialize, Serialize)]
pub struct DomainResearchStatus {
    /// UUID identifier for this research request
    pub request_id: String,
    /// The domain name that was researched
    pub domain: String,
    /// The availability status. See [API Documentation](https://developer.dnsimple.com/v2/domains/research/#getDomainsResearchStatus)
    pub availability: String,
    /// Array of error messages if the domain cannot be researched
    pub errors: Vec<String>,
}

struct DomainResearchStatusEndpoint;

impl Endpoint for DomainResearchStatusEndpoint {
    type Output = DomainResearchStatus;
}

/// The domains research set of endpoints
///
/// See [API Documentation: domains/research](https://developer.dnsimple.com/v2/domains/research)
impl Domains<'_> {
    /// Research a domain name for availability and registration status information.
    ///
    /// This endpoint provides information about a domain's availability status.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let response = client.domains().get_domain_research_status(1234, String::from("example.com")).unwrap();
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `domain`: The domain name to research
    pub fn get_domain_research_status(
        &self,
        account_id: u64,
        domain: String,
    ) -> Result<DNSimpleResponse<DomainResearchStatus>, DNSimpleError> {
        let path = format!("/{}/domains/research/status", account_id);
        let mut filter_map = std::collections::HashMap::new();
        filter_map.insert("domain".to_string(), domain);
        let options = RequestOptions {
            filters: Some(Filters {
                filters: filter_map,
            }),
            sort: None,
            paginate: None,
        };
        self.client
            .get::<DomainResearchStatusEndpoint>(&path, Some(options))
    }
}
