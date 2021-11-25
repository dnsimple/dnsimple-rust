use dnsimple_rust::dnsimple::registrar::{DomainRegistrationPayload, DomainRenewalPayload, DomainTransferPayload};
use crate::common::setup_mock_for;
mod common;

#[test]
fn test_check_domain() {
    let setup = setup_mock_for("/1010/registrar/domains/ruby.codes/check", "checkDomain/success", "GET");
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("ruby.codes");

    let response = client.registrar().check_domain(account_id, domain).unwrap();
    let domain_check = response.data.unwrap();

    assert_eq!("ruby.codes", domain_check.domain);
    assert_eq!(true, domain_check.available);
    assert_eq!(true, domain_check.premium);
}

#[test]
fn test_check_domain_premium_price() {
    let setup = setup_mock_for("/1010/registrar/domains/ruby.codes/premium_price?action=registration", "checkDomainPremiumPrice/success", "GET");
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("ruby.codes");

    let response = client.registrar().check_domain_premium_price(account_id, domain, None).unwrap();
    let domain_premium_price = response.data.unwrap();

    assert_eq!("2640.00", domain_premium_price.premium_price);
    assert_eq!("registration", domain_premium_price.action);
}

#[test]
fn test_check_domain_premium_price_not_a_premium_domain() {
    let setup = setup_mock_for("/1010/registrar/domains/cocotero.love/premium_price?action=registration", "checkDomainPremiumPrice/error_400_not_a_premium_domain", "GET");
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("cocotero.love");

    let response = client.registrar().check_domain_premium_price(account_id, domain, None).unwrap();
    let error = response.errors.unwrap();

    assert_eq!("`cocotero.love` is not a premium domain for registration", error.message.unwrap());
}
#[test]
fn test_check_domain_premium_price_tld_not_supported() {
    let setup = setup_mock_for("/1010/registrar/domains/.love/premium_price?action=registration", "checkDomainPremiumPrice/error_400_tld_not_supported", "GET");
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from(".love");

    let response = client.registrar().check_domain_premium_price(account_id, domain, None).unwrap();
    let error = response.errors.unwrap();

    assert_eq!("TLD .LOVE is not supported", error.message.unwrap());
}

#[test]
fn test_get_domain_prices() {
    let setup = setup_mock_for("/1010/registrar/domains/bingo.pizza/prices", "getDomainPrices/success", "GET");
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("bingo.pizza");

    let response = client.registrar().get_domain_prices(account_id, domain).unwrap();
    let domain_prices = response.data.unwrap();

    assert_eq!("bingo.pizza", domain_prices.domain);
    assert_eq!(true, domain_prices.premium);
    assert_eq!(20.0, domain_prices.registration_price);
    assert_eq!(20.0, domain_prices.renewal_price);
    assert_eq!(20.0, domain_prices.transfer_price);
}

#[test]
fn test_get_domain_prices_failure() {
    let setup = setup_mock_for("/1010/registrar/domains/bingo.pineapple/prices", "getDomainPrices/failure", "GET");
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("bingo.pineapple");

    let response = client.registrar().get_domain_prices(account_id, domain).unwrap();
    let error = response.errors.unwrap();

    assert_eq!("TLD .PINEAPPLE is not supported", error.message.unwrap());
}

#[test]
fn test_register_domain() {
    let setup = setup_mock_for("/1010/registrar/domains/example.com/registrations", "registerDomain/success", "POST");
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("example.com");
    let payload = DomainRegistrationPayload {
        registrant_id: 2,
        whois_privacy: None,
        auto_renew: None,
        extended_attributes: None,
        premium_price: None,
    };

    let response = client.registrar().register_domain(account_id, domain, payload).unwrap();
    let domain_registration = response.data.unwrap();

    assert_eq!(1, domain_registration.id);
    assert_eq!(999, domain_registration.domain_id);
    assert_eq!(2, domain_registration.registrant_id);
    assert_eq!(1, domain_registration.period);
    assert_eq!("new", domain_registration.state);
    assert_eq!(false, domain_registration.auto_renew);
    assert_eq!(false, domain_registration.whois_privacy);
    assert_eq!("2016-12-09T19:35:31Z", domain_registration.created_at);
    assert_eq!("2016-12-09T19:35:31Z", domain_registration.updated_at);
}

#[test]
fn test_transfer_domain() {
    let setup = setup_mock_for("/1010/registrar/domains/example.com/transfers", "transferDomain/success", "POST");
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("example.com");
    let payload = DomainTransferPayload {
        registrant_id: 2,
        auth_code: String::from("THE_AUTH_CODE"),
        whois_privacy: None,
        auto_renew: None,
        extended_attributes: None,
        premium_price: None,
    };

    let response = client.registrar().transfer_domain(account_id, domain, payload).unwrap();
    let domain_transfer = response.data.unwrap();

    assert_eq!(1, domain_transfer.id);
    assert_eq!(999, domain_transfer.domain_id);
    assert_eq!(2, domain_transfer.registrant_id);
    assert_eq!("transferring", domain_transfer.state);
    assert_eq!(false, domain_transfer.auto_renew);
    assert_eq!(false, domain_transfer.whois_privacy);
    assert_eq!("2016-12-09T19:43:41Z", domain_transfer.created_at);
    assert_eq!("2016-12-09T19:43:43Z", domain_transfer.updated_at);
}

#[test]
fn test_transfer_domain_error_in_dnsimple() {
    let setup = setup_mock_for("/1010/registrar/domains/google.com/transfers", "transferDomain/error-indnsimple", "POST");
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("google.com");
    let payload = DomainTransferPayload {
        registrant_id: 2,
        auth_code: String::from("THE_AUTH_CODE"),
        whois_privacy: None,
        auto_renew: None,
        extended_attributes: None,
        premium_price: None,
    };

    let response = client.registrar().transfer_domain(account_id, domain, payload).unwrap();
    let error = response.errors.unwrap();

    assert_eq!("The domain google.com is already in DNSimple and cannot be added", error.message.unwrap());
}

#[test]
fn test_transfer_domain_error_missing_auth_code() {
    let setup = setup_mock_for("/1010/registrar/domains/google.com/transfers", "transferDomain/error-missing-authcode", "POST");
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("google.com");
    let payload = DomainTransferPayload {
        registrant_id: 2,
        auth_code: String::from(""),
        whois_privacy: None,
        auto_renew: None,
        extended_attributes: None,
        premium_price: None,
    };

    let response = client.registrar().transfer_domain(account_id, domain, payload).unwrap();
    let errors = response.errors.unwrap();
    let error_details = errors.errors.unwrap();

    assert_eq!("Validation failed", errors.message.unwrap());
    assert_eq!("You must provide an authorization code for the domain", error_details["base"][0]);
}

#[test]
fn test_retrieve_domain_transfer() {
    let setup = setup_mock_for("/1010/registrar/domains/google.com/transfers/361", "getDomainTransfer/success", "GET");
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("google.com");
    let domain_transfer = 361;

    let response = client.registrar().get_domain_transfer(account_id, domain, domain_transfer).unwrap();
    let transfer = response.data.unwrap();

    assert_eq!(361, transfer.id);
    assert_eq!(182245, transfer.domain_id);
    assert_eq!(2715, transfer.registrant_id);
    assert_eq!("cancelled", transfer.state);
    assert_eq!(false, transfer.auto_renew);
    assert_eq!(false, transfer.whois_privacy);
    assert_eq!("Canceled by customer", transfer.status_description.unwrap());
    assert_eq!("2020-06-05T18:08:00Z", transfer.created_at);
    assert_eq!("2020-06-05T18:10:01Z", transfer.updated_at);
}

#[test]
fn test_cancel_domain_transfer() {
    let setup = setup_mock_for("/1010/registrar/domains/google.com/transfers/361", "cancelDomainTransfer/success", "DELETE");
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("google.com");
    let domain_transfer = 361;

    let response = client.registrar().cancel_domain_transfer(account_id, domain, domain_transfer).unwrap();

    assert_eq!(202, response.status);

    let transfer = response.data.unwrap();

    assert_eq!(361, transfer.id);
    assert_eq!(182245, transfer.domain_id);
    assert_eq!(2715, transfer.registrant_id);
    assert_eq!("transferring", transfer.state);
    assert_eq!(false, transfer.auto_renew);
    assert_eq!(false, transfer.whois_privacy);
    assert_eq!(None, transfer.status_description);
    assert_eq!("2020-06-05T18:08:00Z", transfer.created_at);
    assert_eq!("2020-06-05T18:08:04Z", transfer.updated_at);
}

#[test]
fn test_renew_a_domain() {
    let setup = setup_mock_for("/1010/registrar/domains/example.com/renewals", "renewDomain/success", "POST");
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("example.com");
    let payload = DomainRenewalPayload {
        period: 1,
        premium_price: None
    };

    let response = client.registrar().renew_domain(account_id, domain, payload).unwrap();
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
    let setup = setup_mock_for("/1010/registrar/domains/example.com/renewals", "renewDomain/error-tooearly", "POST");
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("example.com");
    let payload = DomainRenewalPayload {
        period: 1,
        premium_price: None
    };

    let response = client.registrar().renew_domain(account_id, domain, payload).unwrap();
    let errors = response.errors.unwrap();

    assert_eq!("example.com may not be renewed at this time", errors.message.unwrap());
}

#[test]
fn test_authorize_domain_transfer_out() {
    let setup = setup_mock_for("/1010/registrar/domains/example.com/authorize_transfer_out", "authorizeDomainTransferOut/success", "POST");
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("example.com");

    let response = client.registrar().transfer_domain_out(account_id, domain);

    assert_eq!(204, response.status);
}