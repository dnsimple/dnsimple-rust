use crate::common::setup_mock_for;
use dnsimple::dnsimple::oauth::OAuthTokenPayload;

mod common;

#[test]
fn exchange_authorization_for_token_test() {
    let setup = setup_mock_for("/oauth/access_token", "oauthAccessToken/success", "POST");
    let client = setup.0;

    let payload = OAuthTokenPayload {
        client_id: "id".to_string(),
        client_secret: "secret".to_string(),
        code: "code".to_string(),
        redirect_uri: "/redirect_uri".to_string(),
        state: "state".to_string(),
    };

    let access_token = match client.oauth().exchange_authorization_for_token(payload) {
        Ok(token) => token,
        Err(_) => {
            panic!("The token wasn't where we expected it to be")
        }
    };

    assert_eq!(
        "zKQ7OLqF5N1gylcJweA9WodA000BUNJD",
        access_token.access_token
    );
    assert_eq!("Bearer", access_token.token_type);
    assert_eq!(None, access_token.scope);
    assert_eq!(1, access_token.account_id);
}
