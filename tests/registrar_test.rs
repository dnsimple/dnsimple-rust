use crate::common::setup_mock_for;
use dnsimple::dnsimple::registrar::{
    DomainRegistrationPayload, DomainRenewalPayload, DomainTransferPayload,
};
mod common;

#[test]
fn test_check_domain() {
    let setup = setup_mock_for(
        "/1010/registrar/domains/ruby.codes/check",
        "checkDomain/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = "ruby.codes";

    let response = client.registrar().check_domain(account_id, domain).unwrap();
    let domain_check = response.data.unwrap();

    assert_eq!("ruby.codes", domain_check.domain);
    assert!(domain_check.available);
    assert!(domain_check.premium);
}

#[test]
fn test_check_domain_premium_price() {
    let setup = setup_mock_for(
        "/1010/registrar/domains/ruby.codes/premium_price?action=registration",
        "checkDomainPremiumPrice/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = "ruby.codes";

    let response = client
        .registrar()
        .check_domain_premium_price(account_id, domain, None)
        .unwrap();
    let domain_premium_price = response.data.unwrap();

    assert_eq!("2640.00", domain_premium_price.premium_price);
    assert_eq!("registration", domain_premium_price.action);
}

#[test]
fn test_check_domain_premium_price_not_a_premium_domain() {
    let setup = setup_mock_for(
        "/1010/registrar/domains/cocotero.love/premium_price?action=registration",
        "checkDomainPremiumPrice/error_400_not_a_premium_domain",
        "GET",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = "cocotero.love";

    let response = client
        .registrar()
        .check_domain_premium_price(account_id, domain, None);
    assert!(response.is_err());

    let error = response.unwrap_err();

    assert_eq!(
        "`cocotero.love` is not a premium domain for registration",
        error.to_string()
    );
}
#[test]
fn test_check_domain_premium_price_tld_not_supported() {
    let setup = setup_mock_for(
        "/1010/registrar/domains/.love/premium_price?action=registration",
        "checkDomainPremiumPrice/error_400_tld_not_supported",
        "GET",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = ".love";

    let response = client
        .registrar()
        .check_domain_premium_price(account_id, domain, None);
    let error = response.unwrap_err();

    assert_eq!("TLD .LOVE is not supported", error.to_string());
}

#[test]
fn test_get_domain_prices() {
    let setup = setup_mock_for(
        "/1010/registrar/domains/bingo.pizza/prices",
        "getDomainPrices/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = "bingo.pizza";

    let response = client
        .registrar()
        .get_domain_prices(account_id, domain)
        .unwrap();
    let domain_prices = response.data.unwrap();

    assert_eq!("bingo.pizza", domain_prices.domain);
    assert!(domain_prices.premium);
    assert_eq!(20.0, domain_prices.registration_price);
    assert_eq!(20.0, domain_prices.renewal_price);
    assert_eq!(20.0, domain_prices.transfer_price);
}

#[test]
fn test_get_domain_prices_failure() {
    let setup = setup_mock_for(
        "/1010/registrar/domains/bingo.pineapple/prices",
        "getDomainPrices/failure",
        "GET",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = "bingo.pineapple";

    let response = client.registrar().get_domain_prices(account_id, domain);
    let error = response.unwrap_err();

    assert_eq!("TLD .PINEAPPLE is not supported", error.to_string());
}

#[test]
fn test_get_domain_registration() {
    let setup = setup_mock_for(
        "/1010/registrar/domains/bingo.pizza/registrations/1535",
        "getDomainRegistration/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = "bingo.pizza";
    let domain_registration_id = 1535;

    let response = client
        .registrar()
        .get_domain_registration(account_id, domain, domain_registration_id)
        .unwrap();
    let domain_registration = response.data.unwrap();

    assert_eq!(domain_registration.id, 361);
    assert_eq!(domain_registration.domain_id, 104040);
    assert_eq!(domain_registration.registrant_id, 2715);
    assert_eq!(domain_registration.period, 1);
    assert_eq!(domain_registration.state, "registering");
    assert_eq!(domain_registration.auto_renew, false);
    assert_eq!(domain_registration.whois_privacy, false);
    assert_eq!(domain_registration.created_at, "2023-01-27T17:44:32Z");
    assert_eq!(domain_registration.updated_at, "2023-01-27T17:44:40Z");
}

#[test]
fn test_get_domain_renewal() {
    let setup = setup_mock_for(
        "/1010/registrar/domains/bingo.pizza/renewals/1535",
        "getDomainRenewal/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = "bingo.pizza";
    let domain_renewal_id = 1535;

    let response = client
        .registrar()
        .get_domain_renewal(account_id, domain, domain_renewal_id)
        .unwrap();
    let domain_renewal = response.data.unwrap();

    assert_eq!(domain_renewal.id, 1);
    assert_eq!(domain_renewal.domain_id, 999);
    assert_eq!(domain_renewal.period, 1);
    assert_eq!(domain_renewal.state, "renewed");
    assert_eq!(domain_renewal.created_at, "2016-12-09T19:46:45Z");
    assert_eq!(domain_renewal.updated_at, "2016-12-12T19:46:45Z");
}

#[test]
fn test_register_domain() {
    let setup = setup_mock_for(
        "/1010/registrar/domains/example.com/registrations",
        "registerDomain/success",
        "POST",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = "example.com";
    let payload = DomainRegistrationPayload {
        registrant_id: 2,
        whois_privacy: None,
        auto_renew: None,
        extended_attributes: None,
        premium_price: None,
    };

    let response = client
        .registrar()
        .register_domain(account_id, domain, payload)
        .unwrap();
    let domain_registration = response.data.unwrap();

    assert_eq!(1, domain_registration.id);
    assert_eq!(999, domain_registration.domain_id);
    assert_eq!(2, domain_registration.registrant_id);
    assert_eq!(1, domain_registration.period);
    assert_eq!("new", domain_registration.state);
    assert!(!domain_registration.auto_renew);
    assert!(!domain_registration.whois_privacy);
    assert_eq!("2016-12-09T19:35:31Z", domain_registration.created_at);
    assert_eq!("2016-12-09T19:35:31Z", domain_registration.updated_at);
}

#[test]
fn test_transfer_domain() {
    let setup = setup_mock_for(
        "/1010/registrar/domains/example.com/transfers",
        "transferDomain/success",
        "POST",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = "example.com";
    let payload = DomainTransferPayload {
        registrant_id: 2,
        auth_code: String::from("THE_AUTH_CODE"),
        whois_privacy: None,
        auto_renew: None,
        extended_attributes: None,
        premium_price: None,
    };

    let response = client
        .registrar()
        .transfer_domain(account_id, domain, payload)
        .unwrap();
    let domain_transfer = response.data.unwrap();

    assert_eq!(1, domain_transfer.id);
    assert_eq!(999, domain_transfer.domain_id);
    assert_eq!(2, domain_transfer.registrant_id);
    assert_eq!("transferring", domain_transfer.state);
    assert!(!domain_transfer.auto_renew);
    assert!(!domain_transfer.whois_privacy);
    assert_eq!("2016-12-09T19:43:41Z", domain_transfer.created_at);
    assert_eq!("2016-12-09T19:43:43Z", domain_transfer.updated_at);
}

#[test]
fn test_transfer_domain_error_in_dnsimple() {
    let setup = setup_mock_for(
        "/1010/registrar/domains/google.com/transfers",
        "transferDomain/error-indnsimple",
        "POST",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = "google.com";
    let payload = DomainTransferPayload {
        registrant_id: 2,
        auth_code: String::from("THE_AUTH_CODE"),
        whois_privacy: None,
        auto_renew: None,
        extended_attributes: None,
        premium_price: None,
    };

    let response = client
        .registrar()
        .transfer_domain(account_id, domain, payload);
    let error = response.unwrap_err();

    assert_eq!(
        "The domain google.com is already in DNSimple and cannot be added",
        error.to_string()
    );
}

#[test]
fn test_transfer_domain_error_missing_auth_code() {
    let setup = setup_mock_for(
        "/1010/registrar/domains/google.com/transfers",
        "transferDomain/error-missing-authcode",
        "POST",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = "google.com";
    let payload = DomainTransferPayload {
        registrant_id: 2,
        auth_code: String::from(""),
        whois_privacy: None,
        auto_renew: None,
        extended_attributes: None,
        premium_price: None,
    };

    let response = client
        .registrar()
        .transfer_domain(account_id, domain, payload);
    let errors = response.unwrap_err();

    assert_eq!("Validation failed", errors.to_string());
}

#[test]
fn test_retrieve_domain_transfer() {
    let setup = setup_mock_for(
        "/1010/registrar/domains/google.com/transfers/361",
        "getDomainTransfer/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("google.com");
    let domain_transfer = 361;

    let response = client
        .registrar()
        .get_domain_transfer(account_id, domain, domain_transfer)
        .unwrap();
    let transfer = response.data.unwrap();

    assert_eq!(361, transfer.id);
    assert_eq!(182245, transfer.domain_id);
    assert_eq!(2715, transfer.registrant_id);
    assert_eq!("cancelled", transfer.state);
    assert!(!transfer.auto_renew);
    assert!(!transfer.whois_privacy);
    assert_eq!("Canceled by customer", transfer.status_description.unwrap());
    assert_eq!("2020-06-05T18:08:00Z", transfer.created_at);
    assert_eq!("2020-06-05T18:10:01Z", transfer.updated_at);
}

#[test]
fn test_cancel_domain_transfer() {
    let setup = setup_mock_for(
        "/1010/registrar/domains/google.com/transfers/361",
        "cancelDomainTransfer/success",
        "DELETE",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("google.com");
    let domain_transfer = 361;

    let response = client
        .registrar()
        .cancel_domain_transfer(account_id, domain, domain_transfer)
        .unwrap();

    assert_eq!(202, response.status);

    let transfer = response.data.unwrap();

    assert_eq!(361, transfer.id);
    assert_eq!(182245, transfer.domain_id);
    assert_eq!(2715, transfer.registrant_id);
    assert_eq!("transferring", transfer.state);
    assert!(!transfer.auto_renew);
    assert!(!transfer.whois_privacy);
    assert_eq!(None, transfer.status_description);
    assert_eq!("2020-06-05T18:08:00Z", transfer.created_at);
    assert_eq!("2020-06-05T18:08:04Z", transfer.updated_at);
}

#[test]
fn test_renew_a_domain() {
    let setup = setup_mock_for(
        "/1010/registrar/domains/example.com/renewals",
        "renewDomain/success",
        "POST",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("example.com");
    let payload = DomainRenewalPayload {
        period: 1,
        premium_price: None,
    };

    let response = client
        .registrar()
        .renew_domain(account_id, domain, payload)
        .unwrap();
    let domain_renewal = response.data.unwrap();

    assert_eq!(1, domain_renewal.id);
    assert_eq!(999, domain_renewal.domain_id);
    assert_eq!(1, domain_renewal.period);
    assert_eq!("new", domain_renewal.state);
    assert_eq!("2016-12-09T19:46:45Z", domain_renewal.created_at);
    assert_eq!("2016-12-09T19:46:45Z", domain_renewal.updated_at);
}

#[test]
fn test_renew_a_domain_to_early() {
    let setup = setup_mock_for(
        "/1010/registrar/domains/example.com/renewals",
        "renewDomain/error-tooearly",
        "POST",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("example.com");
    let payload = DomainRenewalPayload {
        period: 1,
        premium_price: None,
    };

    let response = client.registrar().renew_domain(account_id, domain, payload);

    let errors = response.unwrap_err();

    assert_eq!(
        "example.com may not be renewed at this time",
        errors.to_string()
    );
}

#[test]
fn test_authorize_domain_transfer_out() {
    let setup = setup_mock_for(
        "/1010/registrar/domains/example.com/authorize_transfer_out",
        "authorizeDomainTransferOut/success",
        "POST",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("example.com");

    let response = client.registrar().transfer_domain_out(account_id, domain);

    assert!(response.is_ok());
    assert_eq!(204, response.unwrap().status);
}
