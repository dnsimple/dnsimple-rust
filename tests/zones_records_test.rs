use dnsimple_rust::dnsimple::zones_records::{ZoneRecordPayload, ZoneRecordUpdatePayload};
use crate::common::setup_mock_for;
mod common;

#[test]
fn list_zone_records_test() {
    let setup = setup_mock_for("/1010/zones/example.com/records", "listZoneRecords/success", "GET");
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";

    let zone_records = client.zones().list_zone_records(account_id, zone, None).unwrap().data.unwrap();

    assert_eq!(5, zone_records.len());

    let zone_record = zone_records.first().unwrap();

    assert_eq!(1, zone_record.id);
    assert_eq!("example.com", zone_record.zone_id);
    assert_eq!(None, zone_record.parent_id);
    assert!(zone_record.name.is_empty());
    assert_eq!("ns1.dnsimple.com admin.dnsimple.com 1458642070 86400 7200 604800 300",
                zone_record.content);
    assert_eq!(3600, zone_record.ttl);
    assert_eq!(None, zone_record.priority);
    assert_eq!("SOA", zone_record.record_type);
    assert_eq!(1, zone_record.regions.len());
    assert_eq!("global", zone_record.regions.first().unwrap());
    assert_eq!(true, zone_record.system_record);
    assert_eq!("2016-03-22T10:20:53Z", zone_record.created_at);
    assert_eq!("2016-10-05T09:26:38Z", zone_record.updated_at);
}

#[test]
fn create_zone_record_test() {
    let setup = setup_mock_for("/1010/zones/example.com/records", "createZoneRecord/created", "POST");
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let payload = ZoneRecordPayload{
        name: "www".to_string(),
        record_type: "A".to_string(),
        content: "127.0.0.1".to_string(),
        ttl: None,
        priority: None,
        regions: None
    };

    let zone_record = client.zones().create_zone_record(account_id, zone, payload).unwrap().data.unwrap();

    assert_eq!(1, zone_record.id);
    assert_eq!("example.com", zone_record.zone_id);
    assert_eq!(None, zone_record.parent_id);
    assert_eq!("www", zone_record.name);
    assert_eq!("127.0.0.1", zone_record.content);
    assert_eq!(600, zone_record.ttl);
    assert_eq!(None, zone_record.priority);
    assert_eq!("A", zone_record.record_type);
    assert_eq!(false, zone_record.system_record);
    assert_eq!(1, zone_record.regions.len());
    assert_eq!("global", zone_record.regions.first().unwrap());
    assert_eq!("2016-01-07T17:45:13Z", zone_record.created_at);
    assert_eq!("2016-01-07T17:45:13Z", zone_record.updated_at);

}
#[test]
fn create_apex_zone_record_test() {
    let setup = setup_mock_for("/1010/zones/example.com/records", "createZoneRecord/created-apex", "POST");
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let payload = ZoneRecordPayload{
        name: "".to_string(),
        record_type: "A".to_string(),
        content: "127.0.0.1".to_string(),
        ttl: None,
        priority: None,
        regions: None
    };

    let zone_record = client.zones().create_zone_record(account_id, zone, payload).unwrap().data.unwrap();

    assert_eq!(1, zone_record.id);
    assert_eq!("example.com", zone_record.zone_id);
    assert_eq!(None, zone_record.parent_id);
    assert_eq!("", zone_record.name);
    assert_eq!("127.0.0.1", zone_record.content);
    assert_eq!(600, zone_record.ttl);
    assert_eq!(None, zone_record.priority);
    assert_eq!("A", zone_record.record_type);
    assert_eq!(false, zone_record.system_record);
    assert_eq!(1, zone_record.regions.len());
    assert_eq!("global", zone_record.regions.first().unwrap());
    assert_eq!("2016-01-07T17:45:13Z", zone_record.created_at);
    assert_eq!("2016-01-07T17:45:13Z", zone_record.updated_at);
}

#[test]
fn get_zone_record_test() {
    let setup = setup_mock_for("/1010/zones/example.com/records/5", "getZoneRecord/success", "GET");
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let record_id = 5;

    let zone_record = client.zones().get_zone_record(account_id, zone, record_id).unwrap().data.unwrap();

    assert_eq!(5, zone_record.id);
    assert_eq!("example.com", zone_record.zone_id);
    assert_eq!(None, zone_record.parent_id);
    assert_eq!("", zone_record.name);
    assert_eq!("mxa.example.com", zone_record.content);
    assert_eq!(600, zone_record.ttl);
    assert_eq!(10, zone_record.priority.unwrap());
    assert_eq!("MX", zone_record.record_type);
    assert_eq!(false, zone_record.system_record);
    assert_eq!(2, zone_record.regions.len());
    assert_eq!("SV1", zone_record.regions[0]);
    assert_eq!("IAD", zone_record.regions[1]);
    assert_eq!("2016-10-05T09:51:35Z", zone_record.created_at);
    assert_eq!("2016-10-05T09:51:35Z", zone_record.updated_at);
}

#[test]
fn update_zone_record_test() {
    let setup = setup_mock_for("/1010/zones/example.com/records/5", "updateZoneRecord/success", "PATCH");
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let record = 5;
    let payload = ZoneRecordUpdatePayload{
        name: Option::from("".to_string()),
        content: Option::from("mxb.example.com".to_string()),
        ttl: Option::from(3600),
        priority: Option::from(20),
        regions: None,
    };

    let zone_record = client.zones().update_zone_record(account_id, zone, record, payload).unwrap().data.unwrap();

    assert_eq!(5, zone_record.id);
    assert_eq!("example.com", zone_record.zone_id);
    assert_eq!(None, zone_record.parent_id);
    assert_eq!("", zone_record.name);
    assert_eq!("mxb.example.com", zone_record.content);
    assert_eq!(3600, zone_record.ttl);
    assert_eq!(20, zone_record.priority.unwrap());
    assert_eq!("MX", zone_record.record_type);
    assert_eq!(false, zone_record.system_record);
    assert_eq!(1, zone_record.regions.len());
    assert_eq!("global", zone_record.regions.first().unwrap());
    assert_eq!("2016-10-05T09:51:35Z", zone_record.created_at);
    assert_eq!("2016-10-05T09:51:35Z", zone_record.updated_at);
}

#[test]
fn delete_zone_record_test() {
    let setup = setup_mock_for("/1010/zones/example.com/records/5", "deleteZoneRecord/success", "DELETE");
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let record = 5;

    let response = client.zones().delete_zone_record(account_id, zone, record);

    assert_eq!(204, response.status);
}

#[test]
fn check_zone_record_distribution() {
    let setup = setup_mock_for("/1010/zones/example.com/records/5/distribution", "checkZoneRecordDistribution/success", "GET");
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let record = 5;

    let distribution = client.zones().check_zone_record_distribution(account_id, zone, record).unwrap().data.unwrap();

    assert_eq!(true, distribution.distributed);
}

#[test]
fn check_zone_record_distribution_failure() {
    let setup = setup_mock_for("/1010/zones/example.com/records/5/distribution", "checkZoneRecordDistribution/failure", "GET");
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let record = 5;

    let distribution = client.zones().check_zone_record_distribution(account_id, zone, record).unwrap().data.unwrap();

    assert_eq!(false, distribution.distributed);
}

#[test]
fn check_zone_record_distribution_error() {
    let setup = setup_mock_for("/1010/zones/example.com/records/5/distribution", "checkZoneRecordDistribution/error", "GET");
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";
    let record = 5;

    let errors = client.zones().check_zone_record_distribution(account_id, zone, record).unwrap().errors.unwrap();

    assert_eq!("Could not query zone, connection timed out", errors.message.unwrap());
}
