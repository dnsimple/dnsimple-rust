use dnsimple::dnsimple::{Client, new_client};
use mockito::{Matcher, Server, ServerGuard};
use std::fs;

/// Creates a mockserver and a client (changing the url of the client
/// to that of the mockserver to capture the requests).
///
/// It builds a response struct for the mock server using the fixture.
///
/// # Arguments
///
/// `fixture`: the path to the fixture inside the `api` directory
/// `path`: the path in the server (i.e. `/whoami`)
/// `method`: the HTTP method we are going to use (GET, POST, DELETE, ...)
///
pub async fn setup_mock_for(path: &str, fixture: &str, method: &str) -> (Client, ServerGuard) {
    setup_mock_with_body_for(path, fixture, method, Matcher::Any).await
}

/// Creates a mockserver and a client like `setup_mock_for`, but the mock
/// only matches requests with a body that matches `request_body`.
pub async fn setup_mock_with_body_for(
    path: &str,
    fixture: &str,
    method: &str,
    request_body: Matcher,
) -> (Client, ServerGuard) {
    let path = format!("/v2{}", path);
    let fixture = format!("./tests/fixtures/v2/api/{}.http", fixture);

    let content =
        fs::read_to_string(fixture.as_str()).expect("Something went wrong: Couldn't read the file");

    let lines = content.trim_end().lines();
    let status = &content[9..12];
    let body = lines.last();

    let mut server = Server::new_async().await;
    server
        .mock(method, path.as_str())
        .match_body(request_body)
        .with_header("X-RateLimit-Limit", "2")
        .with_header("X-RateLimit-Remaining", "2")
        .with_header("X-RateLimit-Reset", "never")
        .with_status(status.parse().unwrap())
        .with_body(body.unwrap())
        .create_async()
        .await;

    let mut client = new_client(true, String::from("some-token")).unwrap();
    client.set_base_url(&server.url());
    (client, server)
}
