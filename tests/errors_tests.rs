use crate::common::setup_mock_for;

mod common;

#[test]
fn validation_error() {
    let setup = setup_mock_for("/whoami", "validation-error", "GET");
    let client = setup.0;

    let response = client.identity().whoami();
    let error = response.unwrap_err();

    assert_eq!("Validation failed", error.to_string());
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

    assert_eq!("Transport Error - 501(Mock Not Found)", error.to_string());
}
