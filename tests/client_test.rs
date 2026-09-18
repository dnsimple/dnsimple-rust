use crate::common::setup_mock_with_body_for;
use dnsimple::dnsimple::Endpoint;
use mockito::Matcher;
use serde_json::{Value, json};
#[allow(dead_code)]
mod common;

struct ValueEndpoint;

impl Endpoint for ValueEndpoint {
    type Output = Value;
}

#[tokio::test]
async fn post_sends_payload_as_json_body() {
    let payload = json!({"url": "https://example.com"});
    let (client, _server) = setup_mock_with_body_for(
        "/test",
        "createWebhook/created",
        "POST",
        Matcher::Json(payload.clone()),
    )
    .await;

    client
        .post::<ValueEndpoint>("/test", payload)
        .await
        .unwrap();
}

#[tokio::test]
async fn post_with_null_payload_sends_no_body() {
    let (client, _server) = setup_mock_with_body_for(
        "/test",
        "createWebhook/created",
        "POST",
        Matcher::Exact(String::new()),
    )
    .await;

    client
        .post::<ValueEndpoint>("/test", Value::Null)
        .await
        .unwrap();
}

#[tokio::test]
async fn put_with_null_payload_sends_no_body() {
    let (client, _server) = setup_mock_with_body_for(
        "/test",
        "createWebhook/created",
        "PUT",
        Matcher::Exact(String::new()),
    )
    .await;

    client
        .put::<ValueEndpoint>("/test", Value::Null)
        .await
        .unwrap();
}

#[tokio::test]
async fn patch_with_null_payload_sends_no_body() {
    let (client, _server) = setup_mock_with_body_for(
        "/test",
        "createWebhook/created",
        "PATCH",
        Matcher::Exact(String::new()),
    )
    .await;

    client
        .patch::<ValueEndpoint>("/test", Value::Null)
        .await
        .unwrap();
}
