use crate::common::setup_mock_for;
mod common;

#[tokio::test]
async fn test_list_tlds() {
    let setup = setup_mock_for("/tlds", "listTlds/success", "GET").await;
    let client = setup.0;

    let response = client.tlds().list_tlds(None).await.unwrap();
    let tlds = response.data.unwrap();

    assert_eq!(2, tlds.len());

    let tld = tlds.first().unwrap();

    assert_eq!("ac", tld.tld);
    assert_eq!(2, tld.tld_type);
    assert!(!tld.whois_privacy);
    assert!(tld.auto_renew_only);
    assert!(!tld.idn);
    assert_eq!(1, tld.minimum_registration);
    assert!(tld.registration_enabled);
    assert!(tld.renewal_enabled);
    assert!(!tld.transfer_enabled);
    assert_eq!("ds", tld.dnssec_interface_type.as_ref().unwrap());
}

#[tokio::test]
async fn test_get_tld() {
    let setup = setup_mock_for("/tlds/com", "getTld/success", "GET").await;
    let client = setup.0;
    let tld = String::from("com");

    let tld = client.tlds().get_tld(tld).await.unwrap().data.unwrap();

    assert_eq!("com", tld.tld);
    assert_eq!(1, tld.tld_type);
    assert!(tld.whois_privacy);
    assert!(!tld.auto_renew_only);
    assert!(tld.idn);
    assert_eq!(1, tld.minimum_registration);
    assert!(tld.registration_enabled);
    assert!(tld.renewal_enabled);
    assert!(tld.transfer_enabled);
    assert_eq!("ds", tld.dnssec_interface_type.unwrap());
}

#[tokio::test]
async fn test_get_tld_extended_attributes() {
    let setup = setup_mock_for(
        "/tlds/com/extended_attributes",
        "getTldExtendedAttributes/success",
        "GET",
    )
    .await;
    let client = setup.0;
    let tld = String::from("com");

    let response = client.tlds().get_tld_extended_attributes(tld).await.unwrap();
    let extended_attributes = response.data.unwrap();

    assert_eq!(4, extended_attributes.len());

    let extended_attribute = extended_attributes.first().unwrap();
    assert_eq!("uk_legal_type", extended_attribute.name);
    assert_eq!(
        "Legal type of registrant contact",
        extended_attribute.description
    );
    assert!(!extended_attribute.required);

    let options = &extended_attribute.options;

    assert_eq!(17, options.len());

    let option = options.first().unwrap();
    assert_eq!("UK Individual", option.title);
    assert_eq!("IND", option.value);
    assert_eq!("UK Individual (our default value)", option.description);
}
