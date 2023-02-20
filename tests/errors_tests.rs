use crate::common::setup_mock_for;
use assert_matches::assert_matches;
use dnsimple::errors::DNSimpleError;
use serde_json::json;

mod common;

#[test]
fn validation_error() {
    let setup = setup_mock_for("/whoami", "validation-error", "GET");
    let client = setup.0;

    let response = client.identity().whoami();
    let error = response.unwrap_err();

    assert_eq!("Validation failed", error.to_string());
    assert_matches!(error, DNSimpleError::BadRequest{ message, attribute_errors } => {
      assert_eq!("Validation failed", message);
      assert_eq!(json!({"address1":["can't be blank"],"city":["can't be blank"],"country":["can't be blank"],"email":["can't be blank","is an invalid email address"],"first_name":["can't be blank"],"last_name":["can't be blank"],"phone":["can't be blank","is probably not a phone number"],"postal_code":["can't be blank"],"state_province":["can't be blank"]}), attribute_errors.unwrap());
    })
}

#[test]
fn not_found() {
    let setup = setup_mock_for("/whoami", "notfound-certificate", "GET");
    let client = setup.0;

    let response = client.identity().whoami();
    let error = response.unwrap_err();

    assert_eq!("Certificate `0` not found", error.to_string());
}

#[test]
fn method_not_allowed() {
    let setup = setup_mock_for("/whoami", "method-not-allowed", "GET");
    let client = setup.0;

    let response = client.identity().whoami();
    let error = response.unwrap_err();

    assert_eq!("Method not Allowed", error.to_string());
}

#[test]
fn bad_gateway() {
    let setup = setup_mock_for("/whoami", "badgateway", "GET");
    let client = setup.0;

    let response = client.identity().whoami();
    let error = response.unwrap_err();

    assert_eq!("Bad Gateway", error.to_string());
}
#[test]
fn transport() {
    let setup = setup_mock_for("/other", "badgateway", "GET");
    let client = setup.0;

    let response = client.identity().whoami();
    let error = response.unwrap_err();

    assert_eq!("Transport Error - 501(Not Implemented)", error.to_string());
}
