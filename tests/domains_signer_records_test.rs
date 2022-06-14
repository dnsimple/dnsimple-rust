use crate::common::setup_mock_for;
use dnsimple::dnsimple::domains_signer_records::DelegationSignerRecordPayload;

mod common;

#[test]
fn test_list_delegation_signer_records() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/ds_records",
        "listDelegationSignerRecords/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1385_u64;
    let domain = "example.com";

    let response = client
        .domains()
        .list_delegation_signer_records(account_id, domain, None)
        .unwrap();
    let signer_records = response.data.unwrap();

    assert_eq!(1, signer_records.len());

    let record = signer_records.first().unwrap();

    assert_eq!(24, record.id);
    assert_eq!(1010, record.domain_id);
    assert_eq!("8", record.algorithm);
    assert_eq!(
        "C1F6E04A5A61FBF65BF9DC8294C363CF11C89E802D926BDAB79C55D27BEFA94F",
        record.digest
    );
    assert_eq!("2", record.digest_type);
    assert_eq!("44620", record.keytag);
    assert_eq!(None, record.public_key);
    assert_eq!("2017-03-03T13:49:58Z", record.created_at);
    assert_eq!("2017-03-03T13:49:58Z", record.updated_at);
}

#[test]
fn test_create_delegation_signer_record() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/ds_records",
        "createDelegationSignerRecord/created",
        "POST",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let payload = DelegationSignerRecordPayload {
        algorithm: String::from("13"),
        digest: String::from("684a1f049d7d082b7f98691657da5a65764913df7f065f6f8c36edf62d66ca03"),
        digest_type: String::from("2"),
        keytag: String::from("2371"),
        public_key: None,
    };

    let record = client
        .domains()
        .create_delegation_signer_record(account_id, domain, payload)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(2, record.id);
    assert_eq!(1010, record.domain_id);
    assert_eq!("13", record.algorithm);
    assert_eq!(
        "684a1f049d7d082b7f98691657da5a65764913df7f065f6f8c36edf62d66ca03",
        record.digest
    );
    assert_eq!("2", record.digest_type);
    assert_eq!("2371", record.keytag);
    assert_eq!(None, record.public_key);
    assert_eq!("2017-03-03T15:24:00Z", record.created_at);
    assert_eq!("2017-03-03T15:24:00Z", record.updated_at);
}

#[test]
fn test_create_delegation_signer_record_validation_error() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/ds_records",
        "createDelegationSignerRecord/validation-error",
        "POST",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let payload = DelegationSignerRecordPayload {
        algorithm: String::from(""),
        digest: String::from(""),
        digest_type: String::from(""),
        keytag: String::from(""),
        public_key: None,
    };

    let response = client
        .domains()
        .create_delegation_signer_record(account_id, domain, payload);

    assert!(response.is_err());

    let err = response.unwrap_err();

    assert_eq!("Validation failed", err.to_string());
}

#[test]
fn test_get_delegation_signer_record() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/ds_records",
        "getDelegationSignerRecord/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let record = client
        .domains()
        .get_delegation_signer_record(account_id, domain)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(24, record.id);
    assert_eq!(1010, record.domain_id);
    assert_eq!("8", record.algorithm);
    assert_eq!(
        "C1F6E04A5A61FBF65BF9DC8294C363CF11C89E802D926BDAB79C55D27BEFA94F",
        record.digest
    );
    assert_eq!("2", record.digest_type);
    assert_eq!("44620", record.keytag);
    assert_eq!(None, record.public_key);
    assert_eq!("2017-03-03T13:49:58Z", record.created_at);
    assert_eq!("2017-03-03T13:49:58Z", record.updated_at);
}

#[test]
fn test_delete_delegation_signer_record() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/ds_records/24",
        "deleteDelegationSignerRecord/success",
        "DELETE",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";
    let delegation_signer_record_id = 24;

    let response = client.domains().delete_delegation_signer_record(
        account_id,
        domain,
        delegation_signer_record_id,
    );

    assert!(response.is_ok());
    assert_eq!(204, response.unwrap().status);
}
