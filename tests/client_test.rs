use crate::common::setup_mock_with_request_for;
use dnsimple::dnsimple::{Client, Endpoint};
use mockito::Matcher;
use serde::Serialize;
use serde_json::{Value, json};
#[allow(dead_code)]
mod common;

struct ValueEndpoint;

impl Endpoint for ValueEndpoint {
    type Output = Value;
}

async fn send(client: &Client, method: &str, payload: impl Serialize) {
    match method {
        "POST" => client.post::<ValueEndpoint>("/test", payload).await,
        "PUT" => client.put::<ValueEndpoint>("/test", payload).await,
        "PATCH" => client.patch::<ValueEndpoint>("/test", payload).await,
        _ => unreachable!("unsupported method: {method}"),
    }
    .unwrap();
}

async fn assert_payload_sends_json_body(method: &str) {
    let payload = json!({"url": "https://example.com"});
    let (client, _server) = setup_mock_with_request_for(
        "/test",
        "createWebhook/created",
        method,
        &[("content-type", Matcher::Exact("application/json".into()))],
        Matcher::Json(payload.clone()),
    )
    .await;

    send(&client, method, payload).await;
}

async fn assert_payload_sends_no_body(method: &str, payload: impl Serialize) {
    let (client, _server) = setup_mock_with_request_for(
        "/test",
        "createWebhook/created",
        method,
        &[
            ("content-type", Matcher::Missing),
            ("content-length", Matcher::Exact("0".into())),
        ],
        Matcher::Exact(String::new()),
    )
    .await;

    send(&client, method, payload).await;
}

#[tokio::test]
async fn post_sends_payload_as_json_body() {
    assert_payload_sends_json_body("POST").await;
}

#[tokio::test]
async fn put_sends_payload_as_json_body() {
    assert_payload_sends_json_body("PUT").await;
}

#[tokio::test]
async fn patch_sends_payload_as_json_body() {
    assert_payload_sends_json_body("PATCH").await;
}

#[tokio::test]
async fn post_with_null_payload_sends_no_body() {
    assert_payload_sends_no_body("POST", Value::Null).await;
}

#[tokio::test]
async fn put_with_null_payload_sends_no_body() {
    assert_payload_sends_no_body("PUT", Value::Null).await;
}

#[tokio::test]
async fn patch_with_null_payload_sends_no_body() {
    assert_payload_sends_no_body("PATCH", Value::Null).await;
}

#[tokio::test]
async fn post_with_unit_payload_sends_no_body() {
    assert_payload_sends_no_body("POST", ()).await;
}

#[tokio::test]
async fn post_with_none_payload_sends_no_body() {
    assert_payload_sends_no_body("POST", None::<Value>).await;
}

#[tokio::test]
async fn empty_post_with_response_sends_no_body() {
    let (client, _server) = setup_mock_with_request_for(
        "/test",
        "createWebhook/created",
        "POST",
        &[("content-length", Matcher::Exact("0".into()))],
        Matcher::Exact(String::new()),
    )
    .await;

    client
        .empty_post_with_response::<ValueEndpoint>("/test")
        .await
        .unwrap();
}
