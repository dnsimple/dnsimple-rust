use crate::common::setup_mock_for;
mod common;

#[tokio::test]
async fn list_webhooks_test() {
    let setup = setup_mock_for("/1010/webhooks", "listWebhooks/success", "GET").await;
    let client = setup.0;
    let account_id = 1010;

    let webhooks = client
        .webhooks()
        .list_webhooks(account_id, None)
        .await
        .unwrap()
        .data
        .unwrap();

    assert_eq!(2, webhooks.len());
}

#[tokio::test]
async fn create_webhook_test() {
    let setup = setup_mock_for("/1010/webhooks", "createWebhook/created", "POST").await;
    let client = setup.0;
    let account_id = 1010;

    let webhook = client
        .webhooks()
        .create_webhook(account_id, String::from("https://webhook.test"))
        .await
        .unwrap()
        .data
        .unwrap();

    assert_eq!(1, webhook.id);
    assert_eq!("https://webhook.test", webhook.url);
}

#[tokio::test]
async fn get_webhook_test() {
    let setup = setup_mock_for("/1010/webhooks/1", "getWebhook/success", "GET").await;
    let client = setup.0;
    let account_id = 1010;
    let webhook_id = String::from("1");

    let webhook = client
        .webhooks()
        .get_webhook(account_id, webhook_id)
        .await
        .unwrap()
        .data
        .unwrap();

    assert_eq!(1, webhook.id);
    assert_eq!("https://webhook.test", webhook.url);
}
#[tokio::test]
async fn delete_webhook_test() {
    let setup = setup_mock_for("/1010/webhooks/1", "deleteWebhook/success", "DELETE").await;
    let client = setup.0;
    let account_id = 1010;
    let webhook_id = String::from("1");

    let response = client.webhooks().delete_webhook(account_id, webhook_id).await;

    assert!(response.is_ok());
    assert_eq!(204, response.unwrap().status);
}
