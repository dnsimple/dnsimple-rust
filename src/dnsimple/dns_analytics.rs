use crate::dnsimple::{Client, Endpoint, Pagination, RequestOptions};
use crate::errors::DNSimpleError;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Represents a row of DNS analytics data
///
/// A field is `None` when the query does not group the results by it.
#[derive(Debug, Deserialize, Serialize)]
pub struct DnsAnalyticsRow {
    /// The zone name.
    pub zone_name: Option<String>,
    /// The date in ISO8601 format (YYYY-MM-DD).
    pub date: Option<String>,
    /// The number of DNS queries.
    pub volume: Option<u64>,
}

/// Represents the query parameters that produced the DNS analytics data
#[derive(Debug, Deserialize, Serialize)]
pub struct DnsAnalyticsQuery {
    /// The account ID.
    pub account_id: u64,
    /// The start date in ISO8601 format (YYYY-MM-DD).
    pub start_date: Option<String>,
    /// The end date in ISO8601 format (YYYY-MM-DD).
    pub end_date: Option<String>,
    /// The sort order.
    pub sort: String,
    /// The page number.
    pub page: u64,
    /// The number of entries per page.
    pub per_page: u64,
    /// The attributes that group the results, separated by a comma.
    pub groupings: Option<String>,
}

/// Represents the response from the DNS analytics query
#[derive(Debug)]
pub struct DnsAnalyticsResponse {
    /// The maximum number of requests you can perform per hour.
    pub rate_limit: String,
    /// The number of requests remaining in the current rate limit window.
    pub rate_limit_remaining: String,
    /// The time at which the current rate limit window in [Unix time](https://en.wikipedia.org/wiki/Unix_time) format.
    pub rate_limit_reset: String,
    /// The HTTP Status Code
    pub status: u16,
    /// The rows of DNS analytics data.
    pub data: Option<Vec<DnsAnalyticsRow>>,
    /// The pagination information.
    pub pagination: Option<Pagination>,
    /// The query parameters that produced the data.
    pub query: Option<DnsAnalyticsQuery>,
    /// The body as a JSON `Value`
    pub body: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct DnsAnalyticsData {
    headers: Vec<String>,
    rows: Vec<Vec<Value>>,
}

impl DnsAnalyticsData {
    fn into_rows(self) -> Result<Vec<DnsAnalyticsRow>, DNSimpleError> {
        self.rows
            .into_iter()
            .map(|row| {
                let fields: Map<String, Value> = self.headers.iter().cloned().zip(row).collect();
                serde_json::from_value(Value::Object(fields))
                    .map_err(|e| DNSimpleError::Deserialization(e.to_string()))
            })
            .collect()
    }
}

struct DnsAnalyticsEndpoint;

impl Endpoint for DnsAnalyticsEndpoint {
    type Output = DnsAnalyticsData;
}

/// The DNS Analytics Service handles the DNS analytics endpoints of the DNSimple API.
///
/// See [API Documentation: DNS analytics](https://developer.dnsimple.com/v2/dns-analytics/)
pub struct DnsAnalytics<'a> {
    pub client: &'a Client,
}

impl DnsAnalytics<'_> {
    /// Queries the DNS analytics data for the account.
    ///
    /// The DNS Analytics API is in Public Beta.
    ///
    /// See [API Documentation](https://developer.dnsimple.com/v2/dns-analytics/#queryDnsAnalytics)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::{new_client, Filters, RequestOptions, Sort};
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() {
    ///     let client = new_client(true, String::from("AUTH_TOKEN")).unwrap();
    ///     let mut filters = HashMap::new();
    ///     filters.insert(String::from("groupings"), String::from("zone_name,date"));
    ///     let options = RequestOptions {
    ///         filters: Some(Filters::new(filters)),
    ///         sort: Some(Sort::new(String::from("date:asc"))),
    ///         paginate: None,
    ///     };
    ///     let response = client.dns_analytics().query(1234, Some(options)).await.unwrap();
    ///     let rows = response.data.unwrap();
    /// }
    /// ```
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `options`: The `RequestOptions`
    ///            - Filters: `start_date`, `end_date`, `groupings`
    ///            - Sort: `date`, `zone_name`, `volume`
    pub async fn query(
        &self,
        account_id: u64,
        options: Option<RequestOptions>,
    ) -> Result<DnsAnalyticsResponse, DNSimpleError> {
        let path = format!("/{}/dns_analytics", account_id);

        let response = self
            .client
            .get::<DnsAnalyticsEndpoint>(&path, options)
            .await?;
        let data = response.data.map(DnsAnalyticsData::into_rows).transpose()?;
        let query = response
            .body
            .as_ref()
            .and_then(|body| body.get("query"))
            .cloned()
            .map(serde_json::from_value)
            .transpose()
            .map_err(|e| DNSimpleError::Deserialization(e.to_string()))?;

        Ok(DnsAnalyticsResponse {
            rate_limit: response.rate_limit,
            rate_limit_remaining: response.rate_limit_remaining,
            rate_limit_reset: response.rate_limit_reset,
            status: response.status,
            data,
            pagination: response.pagination,
            query,
            body: response.body,
        })
    }
}
