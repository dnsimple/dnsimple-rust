use crate::common::setup_mock_for;
use assert_matches::assert_matches;
use dnsimple::dnsimple::zones_records::{
    ZoneRecordBatchCreate, ZoneRecordBatchDelete, ZoneRecordBatchUpdate, ZoneRecordPayload,
    ZoneRecordUpdatePayload, ZoneRecordsBatchChangePayload,
};
use dnsimple::errors::DNSimpleError;
use serde_json::json;
mod common;

#[allow(deprecated)]
#[tokio::test]
async fn list_zone_records_test() {
    let setup = setup_mock_for(
        "/1010/zones/example.com/records",
        "listZoneRecords/success",
        "GET",
    )
    .await;
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";

    let zone_records = client
        .zones()
        .list_zone_records(account_id, zone, None)
        .await
        .unwrap()
        .data
        .unwrap();

    assert_eq!(5, zone_records.len());

    let zone_record = zone_records.first().unwrap();

    assert_eq!(1, zone_record.id);
    assert_eq!("example.com", zone_record.zone_id);
    assert_eq!(None, zone_record.parent_id);
    assert!(zone_record.name.is_empty());
    assert_eq!(
        "ns1.dnsimple.com admin.dnsimple.com 1458642070 86400 7200 604800 300",
        zone_record.content
    );
    assert_eq!(3600, zone_record.ttl);
    assert_eq!(None, zone_record.priority);
    assert_eq!("SOA", zone_record.record_type);
    assert_eq!(1, zone_record.regions.as_ref().unwrap().len());
    assert_eq!(
        "global",
        zone_record.regions.as_ref().unwrap().first().unwrap()
    );
    assert!(zone_record.system_record);
    assert_eq!("2016-03-22T10:20:53Z", zone_record.created_at);
    assert_eq!("2016-10-05T09:26:38Z", zone_record.updated_at);
}

#[allow(deprecated)]
#[tokio::test]
async fn create_zone_record_test() {
    let setup = setup_mock_for(
        "/1010/zones/example.com/records",
        "createZoneRecord/created",
        "POST",
    )
    .await;
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let payload = ZoneRecordPayload {
        name: "www".to_string(),
        record_type: "A".to_string(),
        content: "127.0.0.1".to_string(),
        ttl: None,
        priority: None,
        regions: None,
    };

    let zone_record = client
        .zones()
        .create_zone_record(account_id, zone, payload)
        .await
        .unwrap()
        .data
        .unwrap();

    assert_eq!(1, zone_record.id);
    assert_eq!("example.com", zone_record.zone_id);
    assert_eq!(None, zone_record.parent_id);
    assert_eq!("www", zone_record.name);
    assert_eq!("127.0.0.1", zone_record.content);
    assert_eq!(600, zone_record.ttl);
    assert_eq!(None, zone_record.priority);
    assert_eq!("A", zone_record.record_type);
    assert!(!zone_record.system_record);
    let regions = zone_record.regions.unwrap();
    assert_eq!(1, regions.len());
    assert_eq!("global", regions.first().unwrap());
    assert_eq!("2016-01-07T17:45:13Z", zone_record.created_at);
    assert_eq!("2016-01-07T17:45:13Z", zone_record.updated_at);
}
#[allow(deprecated)]
#[tokio::test]
async fn create_apex_zone_record_test() {
    let setup = setup_mock_for(
        "/1010/zones/example.com/records",
        "createZoneRecord/created-apex",
        "POST",
    )
    .await;
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let payload = ZoneRecordPayload {
        name: "".to_string(),
        record_type: "A".to_string(),
        content: "127.0.0.1".to_string(),
        ttl: None,
        priority: None,
        regions: None,
    };

    let zone_record = client
        .zones()
        .create_zone_record(account_id, zone, payload)
        .await
        .unwrap()
        .data
        .unwrap();

    assert_eq!(1, zone_record.id);
    assert_eq!("example.com", zone_record.zone_id);
    assert_eq!(None, zone_record.parent_id);
    assert_eq!("", zone_record.name);
    assert_eq!("127.0.0.1", zone_record.content);
    assert_eq!(600, zone_record.ttl);
    assert_eq!(None, zone_record.priority);
    assert_eq!("A", zone_record.record_type);
    assert!(!zone_record.system_record);
    let regions = zone_record.regions.unwrap();
    assert_eq!(1, regions.len());
    assert_eq!("global", regions.first().unwrap());
    assert_eq!("2016-01-07T17:45:13Z", zone_record.created_at);
    assert_eq!("2016-01-07T17:45:13Z", zone_record.updated_at);
}

#[allow(deprecated)]
#[tokio::test]
async fn get_zone_record_test() {
    let setup = setup_mock_for(
        "/1010/zones/example.com/records/5",
        "getZoneRecord/success",
        "GET",
    )
    .await;
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let record_id = 5;

    let zone_record = client
        .zones()
        .get_zone_record(account_id, zone, record_id)
        .await
        .unwrap()
        .data
        .unwrap();

    assert_eq!(5, zone_record.id);
    assert_eq!("example.com", zone_record.zone_id);
    assert_eq!(None, zone_record.parent_id);
    assert_eq!("", zone_record.name);
    assert_eq!("mxa.example.com", zone_record.content);
    assert_eq!(600, zone_record.ttl);
    assert_eq!(10, zone_record.priority.unwrap());
    assert_eq!("MX", zone_record.record_type);
    assert!(!zone_record.system_record);
    let regions = zone_record.regions.unwrap();
    assert_eq!(2, regions.len());
    assert_eq!("SV1", regions[0]);
    assert_eq!("IAD", regions[1]);
    assert_eq!("2016-10-05T09:51:35Z", zone_record.created_at);
    assert_eq!("2016-10-05T09:51:35Z", zone_record.updated_at);
}

#[allow(deprecated)]
#[tokio::test]
async fn update_zone_record_test() {
    let setup = setup_mock_for(
        "/1010/zones/example.com/records/5",
        "updateZoneRecord/success",
        "PATCH",
    )
    .await;
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let record = 5;
    let payload = ZoneRecordUpdatePayload {
        name: Option::from("".to_string()),
        content: Option::from("mxb.example.com".to_string()),
        ttl: Option::from(3600),
        priority: Option::from(20),
        regions: None,
    };

    let zone_record = client
        .zones()
        .update_zone_record(account_id, zone, record, payload)
        .await
        .unwrap()
        .data
        .unwrap();

    assert_eq!(5, zone_record.id);
    assert_eq!("example.com", zone_record.zone_id);
    assert_eq!(None, zone_record.parent_id);
    assert_eq!("", zone_record.name);
    assert_eq!("mxb.example.com", zone_record.content);
    assert_eq!(3600, zone_record.ttl);
    assert_eq!(20, zone_record.priority.unwrap());
    assert_eq!("MX", zone_record.record_type);
    assert!(!zone_record.system_record);
    let regions = zone_record.regions.unwrap();
    assert_eq!(1, regions.len());
    assert_eq!("global", regions.first().unwrap());
    assert_eq!("2016-10-05T09:51:35Z", zone_record.created_at);
    assert_eq!("2016-10-05T09:51:35Z", zone_record.updated_at);
}

#[tokio::test]
async fn delete_zone_record_test() {
    let setup = setup_mock_for(
        "/1010/zones/example.com/records/5",
        "deleteZoneRecord/success",
        "DELETE",
    )
    .await;
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let record = 5;

    let response = client
        .zones()
        .delete_zone_record(account_id, zone, record)
        .await;

    assert!(response.is_ok());
    assert_eq!(204, response.unwrap().status);
}

#[tokio::test]
async fn batch_change_zone_records_test() {
    let setup = setup_mock_for(
        "/1010/zones/example.com/batch",
        "batchChangeZoneRecords/success",
        "POST",
    )
    .await;
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let payload = ZoneRecordsBatchChangePayload {
        creates: Some(vec![ZoneRecordBatchCreate {
            name: "ab".to_string(),
            record_type: "A".to_string(),
            content: "3.2.3.4".to_string(),
            ttl: None,
            priority: None,
            regions: None,
        }]),
        updates: Some(vec![ZoneRecordBatchUpdate {
            id: 67622534,
            name: None,
            content: Some("3.2.3.40".to_string()),
            ttl: None,
            priority: None,
            regions: None,
        }]),
        deletes: Some(vec![ZoneRecordBatchDelete { id: 67622509 }]),
    };

    assert_eq!(
        json!({
            "creates": [{"name": "ab", "type": "A", "content": "3.2.3.4"}],
            "updates": [{"id": 67622534, "content": "3.2.3.40"}],
            "deletes": [{"id": 67622509}]
        }),
        serde_json::to_value(&payload).unwrap()
    );

    let batch_change = client
        .zones()
        .batch_change_zone_records(account_id, zone, payload)
        .await
        .unwrap()
        .data
        .unwrap();

    assert_eq!(2, batch_change.creates.len());
    let created = batch_change.creates.first().unwrap();
    assert_eq!(67623409, created.id);
    assert_eq!("example.com", created.zone_id);
    assert_eq!("ab", created.name);
    assert_eq!("3.2.3.4", created.content);
    assert_eq!(3600, created.ttl);
    assert_eq!(None, created.priority);
    assert_eq!("A", created.record_type);
    assert_eq!(vec!["global"], created.regions.clone().unwrap());
    assert!(!created.system_record);
    assert_eq!("2025-09-05T05:25:00Z", created.created_at);
    assert_eq!("2025-09-05T05:25:00Z", created.updated_at);
    assert_eq!(67623410, batch_change.creates[1].id);

    assert_eq!(2, batch_change.updates.len());
    let updated = batch_change.updates.first().unwrap();
    assert_eq!(67622534, updated.id);
    assert_eq!("update1-1757049890", updated.name);
    assert_eq!("3.2.3.40", updated.content);
    assert_eq!("2025-09-05T04:40:15Z", updated.created_at);
    assert_eq!("2025-09-05T05:25:00Z", updated.updated_at);
    assert_eq!(67622537, batch_change.updates[1].id);

    assert_eq!(2, batch_change.deletes.len());
    assert_eq!(67622509, batch_change.deletes[0].id);
    assert_eq!(67622527, batch_change.deletes[1].id);
}

#[test]
fn batch_change_zone_records_empty_payload_test() {
    let payload = ZoneRecordsBatchChangePayload::default();

    assert_eq!(json!({}), serde_json::to_value(&payload).unwrap());
}

#[tokio::test]
async fn batch_change_zone_records_validation_error_test() {
    let setup = setup_mock_for(
        "/1010/zones/example.com/batch",
        "batchChangeZoneRecords/error_400_create_validation_failed",
        "POST",
    )
    .await;
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let payload = ZoneRecordsBatchChangePayload {
        creates: Some(vec![ZoneRecordBatchCreate {
            name: "ab".to_string(),
            record_type: "SPF".to_string(),
            content: "v=spf1 -all".to_string(),
            ttl: None,
            priority: None,
            regions: None,
        }]),
        ..Default::default()
    };

    let error = client
        .zones()
        .batch_change_zone_records(account_id, zone, payload)
        .await
        .unwrap_err();

    assert_eq!("Validation failed", error.to_string());
    assert_matches!(error, DNSimpleError::BadRequest { message, attribute_errors } => {
        assert_eq!("Validation failed", message);
        assert_eq!(
            json!({"creates": [{"index": 0, "message": "Validation failed", "errors": {"record_type": ["unsupported"]}}]}),
            attribute_errors.unwrap()
        );
    });
}

#[tokio::test]
async fn batch_change_zone_records_delete_not_found_test() {
    let setup = setup_mock_for(
        "/1010/zones/example.com/batch",
        "batchChangeZoneRecords/error_400_delete_validation_failed",
        "POST",
    )
    .await;
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let payload = ZoneRecordsBatchChangePayload {
        deletes: Some(vec![ZoneRecordBatchDelete { id: 67622509 }]),
        ..Default::default()
    };

    let error = client
        .zones()
        .batch_change_zone_records(account_id, zone, payload)
        .await
        .unwrap_err();

    assert_matches!(error, DNSimpleError::BadRequest { message, attribute_errors } => {
        assert_eq!("Validation failed", message);
        assert_eq!(
            json!({"deletes": [{"index": 0, "message": "Record not found ID=67622509"}]}),
            attribute_errors.unwrap()
        );
    });
}

#[tokio::test]
async fn check_zone_record_distribution() {
    let setup = setup_mock_for(
        "/1010/zones/example.com/records/5/distribution",
        "checkZoneRecordDistribution/success",
        "GET",
    )
    .await;
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let record = 5;

    let distribution = client
        .zones()
        .check_zone_record_distribution(account_id, zone, record)
        .await
        .unwrap()
        .data
        .unwrap();

    assert!(distribution.distributed);
}

#[tokio::test]
async fn check_zone_record_distribution_failure() {
    let setup = setup_mock_for(
        "/1010/zones/example.com/records/5/distribution",
        "checkZoneRecordDistribution/failure",
        "GET",
    )
    .await;
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let record = 5;

    let distribution = client
        .zones()
        .check_zone_record_distribution(account_id, zone, record)
        .await
        .unwrap()
        .data
        .unwrap();

    assert!(!distribution.distributed);
}

#[tokio::test]
async fn check_zone_record_distribution_error() {
    let setup = setup_mock_for(
        "/1010/zones/example.com/records/5/distribution",
        "checkZoneRecordDistribution/error",
        "GET",
    )
    .await;
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let record = 5;

    let errors = client
        .zones()
        .check_zone_record_distribution(account_id, zone, record)
        .await
        .unwrap_err();

    assert_eq!(
        "Could not query zone, connection timed out",
        errors.to_string()
    );
}
