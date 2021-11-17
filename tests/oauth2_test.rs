use dnsimple_rust::dnsimple::oauth::OAuthExchange;
use crate::common::setup_mock_for;

mod common;

#[test]
fn access_token_test() {
    let setup = setup_mock_for("/oauth/access_token", "oauthAccessToken/success", "POST");
    let client = setup.0;

    let response = client.oauth().exchange_authorization_token_for(OAuthExchange{
        grant_type: String::from(""),
        client_id: String::from("1"),
        client_secret: String::from(""),
        code: String::from(""),
        redirect_uri: String::from(""),
        state: String::from("")
    }).data;

    match response {
        None => panic!("We should have a payload here."),
        Some(token) => {
            assert_eq!(token.access_token, "zKQ7OLqF5N1gylcJweA9WodA000BUNJD");
            assert_eq!(token.token_type, "Bearer");
            assert_eq!(token.account_id, 1);
        }
    }
}

#[test]
fn access_token_test_error() {
    let setup = setup_mock_for("/oauth/access_token", "oauthAccessToken/error-invalid-request", "POST");
    let client = setup.0;

    let response = client.oauth().exchange_authorization_token_for(OAuthExchange{
        grant_type: String::from(""),
        client_id: String::from(""),
        client_secret: String::from(""),
        code: String::from(""),
        redirect_uri: String::from(""),
        state: String::from("")
    });

    assert_eq!(response.message.as_ref().unwrap().error, "invalid_request");
    assert_eq!(response.message.unwrap().error_description,"Invalid \"state\": value doesn't match the \"state\" in the authorization request");
}