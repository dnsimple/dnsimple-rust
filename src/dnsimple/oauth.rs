use crate::dnsimple::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OAuthTokenPayload {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    pub redirect_uri: String,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessToken {
    pub access_token: String,
    pub account_id: u64,
    pub scope: Option<String>,
    pub token_type: String,
}

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
    /// use dnsimple_rust::dnsimple::new_client;
    /// use dnsimple_rust::dnsimple::oauth::OAuthTokenPayload;
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