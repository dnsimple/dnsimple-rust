use crate::common::setup_mock_for;
mod common;

#[test]
fn domain_research_status_test() {
    let setup = setup_mock_for(
        "/1385/domains/research/status?domain=taken.com",
        "getDomainsResearchStatus/success-unavailable",
        "GET",
    );
    let client = setup.0;
    let account_id = 1385_u64;

    let response = client
        .domains()
        .domain_research_status(account_id, String::from("taken.com"))
        .unwrap();

    let data = response.data.unwrap();
    assert_eq!("25dd77cb-2f71-48b9-b6be-1dacd2881418", data.request_id);
    assert_eq!("taken.com", data.domain);
    assert_eq!("unavailable", data.availability);
    assert_eq!(Vec::<String>::new(), data.errors);
}
