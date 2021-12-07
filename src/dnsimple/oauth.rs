use crate::dnsimple::Client;
use serde::{Deserialize, Serialize};

/// Represents the payload used to exchange this information for the
/// access token (`AccessToken`).
#[derive(Debug, Deserialize, Serialize)]
pub struct OAuthTokenPayload {
    /// The client ID you received from DNSimple when you registered the application.
    pub client_id: String,
    /// The client secret you received from DNSimple when you registered the application.
    pub client_secret: String,
    /// The code acquired in the previous authorization step.
    pub code: String,
    /// Only used to validate that it matches the original /oauth/authorize, not used to redirect again.
    pub redirect_uri: String,
    /// The state content originally passed to /oauth/authorize.
    pub state: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct OAuthTokenParams {
    grant_type: String,
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
    state: String,
}

/// Represents an access token containing the token to access the API
#[derive(Debug, Deserialize, Serialize)]
pub struct AccessToken {
    /// The token you can use to authenticate.
    pub access_token: String,
    /// The account ID in DNSimple this token belongs to.
    pub account_id: u64,
    /// The token scope (not used for now).
    pub scope: Option<String>,
    /// The token type.
    pub token_type: String,
}

/// The Oauth Service is used to request access to the API
///
/// See [API Documentation: oauth](https://developer.dnsimple.com/v2/oauth/)
pub struct OAuth<'a> {
    pub client: &'a Client
}

impl OAuth<'_> {
    /// Exchange the short-lived authorization code for an access token
    /// you can use to authenticate your API calls.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dnsimple::dnsimple::new_client;
    /// use dnsimple::dnsimple::oauth::OAuthTokenPayload;
    ///
    /// let client = new_client(true, String::from("AUTH_TOKEN"));
    /// let payload = OAuthTokenPayload {
    ///     client_id: "id".to_string(),
    ///     client_secret: "secret".to_string(),
    ///     code: "code".to_string(),
    ///     redirect_uri: "/redirect_uri".to_string(),
    ///     state: "state".to_string()
    /// };
    ///
    /// let access_token = client.oauth().exchange_authorization_for_token(payload);
    /// ```
    ///
    /// # Attributes
    ///
    /// `payload`: The `OAuthTokenPayload` with the necessary information to get the access token.
    pub fn exchange_authorization_for_token(&self, payload: OAuthTokenPayload) -> AccessToken {
        let path = "/oauth/access_token";
        let params = OAuthTokenParams {
            grant_type: "authorization_code".to_string(),
            client_id: payload.client_id,
            client_secret: payload.client_secret,
            code: payload.code,
            redirect_uri: payload.redirect_uri,
            state: payload.state
        };

        let response = self.client._agent.post(&*self.client.url(path))
            .send_json(serde_json::to_value(params).unwrap());
        response.unwrap().into_json::<AccessToken>().unwrap()
    }
}