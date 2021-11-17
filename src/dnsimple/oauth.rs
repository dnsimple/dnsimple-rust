use serde::{Deserialize, Serialize};

use crate::dnsimple::{Client, dnsimple_error_from, DNSimpleResponse};

/// Represents the payload to be sent to the DNSimple API to get the `AccessToken`
///
/// See [API Documentation: oauth](https://developer.dnsimple.com/v2/oauth/)
#[derive(Debug, Deserialize, Serialize)]
pub struct OAuthExchange {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    pub redirect_uri: String,
    pub state: String,
}

/// Represents the access token data returned by the DNSimple API
///
/// See [API Documentation: oauth](https://developer.dnsimple.com/v2/oauth/)
#[derive(Debug, Deserialize, Serialize)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub scope: Option<String>,
    pub account_id: u64,
}

/// The OAuth Service is used to exchange the code with a bearer token you can use to authenticate
/// to the DNSimple API.
///
/// See [API Documentation: oauth](https://developer.dnsimple.com/v2/oauth/)
pub struct Oauth<'a> {
    pub client: &'a Client
}

impl Oauth<'_> {
    /// Returns the access token used for API calls to the DNSimple API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple_rust::dnsimple::new_client;
    /// use dnsimple_rust::dnsimple::oauth::OAuthExchange;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let oauth_exchange = OAuthExchange {
    ///     grant_type: "".to_string(),
    ///     client_id: "42".to_string(),
    ///     client_secret: "shhh".parse().unwrap(),
    ///     code: "12".parse().unwrap(),
    ///     redirect_uri: "".parse().unwrap(),
    ///     state: "some_state".parse().unwrap()
    /// };
    /// let response = client.oauth().exchange_authorization_token_for(oauth_exchange);
    /// ```
    ///
    /// # Arguments
    ///
    /// `oauth_exchange`: The `OAuthExchange` struct.
    pub fn exchange_authorization_token_for(&self, oauth_exchange: OAuthExchange) -> DNSimpleResponse<AccessToken> {
        let api_response = self.client.post("/oauth/access_token", serde_json::to_value(oauth_exchange).unwrap());
        let raw_response = api_response.raw_http_response;
        let mut dnsimple_response = api_response.response;

        if raw_response.status() < 399 {
            dnsimple_response.data = raw_response.into_json().unwrap();
        } else {
            dnsimple_response.message = dnsimple_error_from(raw_response);
        }

        dnsimple_response
    }
}