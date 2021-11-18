use std::fs;
use mockito::{mock, Mock};
use dnsimple_rust::dnsimple::{Client, new_client};

/// Creates a mockserver and a client (changing the url of the client
/// to that of the mockserver to capture the requests).
///
/// It builds a response struct for the mock server using the fixture.
///
/// # Arguments
///
/// `fixture`: the path to the fixture inside the `api` directory
/// `path`: the path in the server (i.e. `/whoami`)
///
pub fn setup_mock_for(path: &str, fixture: &str, method: &str) -> (Client, Mock) {
    let path = format!("/v2{}", path);
    let fixture = format!("./tests/fixtures/v2/api/{}.http", fixture);
    // println!("We are trying to read this file: {}", fixture);

    let content = fs::read_to_string(fixture.as_str())
        .expect("Something went wrong: Couldn't read the file");
    let tokens = content.split("\r\n\r\n");
    let vec = tokens.collect::<Vec<&str>>();
    let headers = vec.first().unwrap();
    let status = &headers[9..12];
    let body = vec.last();
    // println!("Body returned: {}", body.as_ref().unwrap());

    let mock = mock(method, path.as_str())
        .with_header("X-RateLimit-Limit", "2")
        .with_header("X-RateLimit-Remaining", "2")
        .with_header("X-RateLimit-Reset", "never")
        .with_status(status.parse().unwrap())
        .with_body(body.unwrap()).create();

    let mut client = new_client(true, String::from("some-token"));
    client.set_base_url(&mockito::server_url());
    (client, mock)
}