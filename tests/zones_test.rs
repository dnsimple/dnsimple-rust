use crate::common::setup_mock_for;
mod common;

#[test]
fn list_zones_test() {
    let setup = setup_mock_for("/1010/zones", "listZones/success", "GET");
    let client = setup.0;
    let account_id = 1010;

    let zones = client
        .zones()
        .list_zones(account_id, None)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(2, zones.len());

    let zone = zones.first().unwrap();

    assert_eq!(1, zone.id);
    assert_eq!(1010, zone.account_id);
    assert_eq!("example-alpha.com", zone.name);
    assert!(!zone.reverse);
    assert_eq!("2015-04-23T07:40:03Z", zone.created_at);
    assert_eq!("2015-04-23T07:40:03Z", zone.updated_at);
}

#[test]
fn get_zone_test() {
    let setup = setup_mock_for("/1010/zones/example-alpha.com", "getZone/success", "GET");
    let client = setup.0;
    let account_id = 1010;
    let zone = "example-alpha.com";

    let zone = client
        .zones()
        .get_zone(account_id, zone)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(1, zone.id);
    assert_eq!(1010, zone.account_id);
    assert_eq!("example-alpha.com", zone.name);
    assert!(!zone.reverse);
    assert_eq!("2015-04-23T07:40:03Z", zone.created_at);
    assert_eq!("2015-04-23T07:40:03Z", zone.updated_at);
}

#[test]
fn get_zone_file_test() {
    let setup = setup_mock_for("/1010/zones/example.com/file", "getZoneFile/success", "GET");
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";

    let zone_file = client
        .zones()
        .get_zone_file(account_id, zone)
        .unwrap()
        .data
        .unwrap();

    assert_eq!("$ORIGIN example.com.\n$TTL 1h\nexample.com. 3600 IN SOA ns1.dnsimple.com. admin.dnsimple.com. 1453132552 86400 7200 604800 300\nexample.com. 3600 IN NS ns1.dnsimple.com.\nexample.com. 3600 IN NS ns2.dnsimple.com.\nexample.com. 3600 IN NS ns3.dnsimple.com.\nexample.com. 3600 IN NS ns4.dnsimple.com.\n",
               zone_file.zone);
}

#[test]
fn check_zone_distribution() {
    let setup = setup_mock_for(
        "/1010/zones/example.com/distribution",
        "checkZoneDistribution/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";

    let zone_distribution = client
        .zones()
        .check_zone_distribution(account_id, zone)
        .unwrap()
        .data
        .unwrap();

    assert!(zone_distribution.distributed);
}

#[test]
fn check_zone_distribution_failure() {
    let setup = setup_mock_for(
        "/1010/zones/example.com/distribution",
        "checkZoneDistribution/failure",
        "GET",
    );
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";

    let zone_distribution = client
        .zones()
        .check_zone_distribution(account_id, zone)
        .unwrap()
        .data
        .unwrap();

    assert!(!zone_distribution.distributed);
}

#[test]
fn check_zone_distribution_error() {
    let setup = setup_mock_for(
        "/1010/zones/example.com/distribution",
        "checkZoneDistribution/error",
        "GET",
    );
    let client = setup.0;
    let account_id = 1010;
    let zone = "example.com";

    let response = client.zones().check_zone_distribution(account_id, zone);

    assert_eq!(
        "Could not query zone, connection timed out",
        response.unwrap_err().to_string()
    );
}
