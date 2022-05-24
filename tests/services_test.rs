use crate::common::setup_mock_for;
mod common;

#[test]
fn list_services_test() {
    let setup = setup_mock_for("/services", "listServices/success", "GET");
    let client = setup.0;

    let services = client.services().list_services(None).unwrap().data.unwrap();

    assert_eq!(2, services.len());

    let service = services.last().unwrap();
    let settings = service.settings.first().unwrap();

    assert_eq!(2, service.id);
    assert_eq!("Service 2", service.name);
    assert_eq!("service2", service.sid);
    assert_eq!("Second service example.", service.description);
    assert_eq!(None, service.setup_description);
    assert!(service.requires_setup);
    assert_eq!(None, service.default_subdomain);
    assert_eq!("2014-02-14T19:15:19Z", service.created_at);
    assert_eq!("2016-03-04T09:23:27Z", service.updated_at);
    assert_eq!("username", settings.name);
    assert_eq!("Service 2 Account Username", settings.label);
    assert_eq!(Some(".service2.com".to_string()), settings.append);
    assert_eq!(
        "Your Service2 username is used to connect services to your account.",
        settings.description
    );
    assert_eq!(Some("username".to_string()), settings.example);
    assert!(!settings.password);
}

#[test]
fn get_service_test() {
    let setup = setup_mock_for("/services/1", "getService/success", "GET");
    let client = setup.0;
    let service_id = "1";

    let service = client
        .services()
        .get_service(String::from(service_id))
        .unwrap()
        .data
        .unwrap();

    assert_eq!(1, service.id);
    assert_eq!("Service 1", service.name);
    assert_eq!("service1", service.sid);
    assert_eq!("First service example.", service.description);
    assert_eq!(None, service.setup_description);
    assert!(service.requires_setup);
    assert_eq!(None, service.default_subdomain);
    assert_eq!("2014-02-14T19:15:19Z", service.created_at);
    assert_eq!("2016-03-04T09:23:27Z", service.updated_at);
    assert_eq!("username", service.settings.first().unwrap().name);
    assert_eq!(
        "Service 1 Account Username",
        service.settings.first().unwrap().label
    );
    assert_eq!(
        Some(".service1.com".to_string()),
        service.settings.first().unwrap().append
    );
    assert_eq!(
        "Your Service 1 username is used to connect services to your account.",
        service.settings.first().unwrap().description
    );
    assert_eq!(
        Some("username".to_string()),
        service.settings.first().unwrap().example
    );
    assert!(!service.settings.first().unwrap().password);
}

#[test]
fn applied_services_test() {
    let setup = setup_mock_for(
        "/1010/domains/example.com/services",
        "appliedServices/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = "example.com";

    let applied_services = client
        .services()
        .applied_services(account_id, String::from(domain), None)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(1, applied_services.len());
}

#[test]
fn apply_service_test() {
    let setup = setup_mock_for(
        "/1010/domains/example.com/services/wordpress",
        "applyService/success",
        "POST",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = "example.com";
    let service = "wordpress";

    let response =
        client
            .services()
            .apply_service(account_id, String::from(domain), String::from(service));

    assert!(response.is_ok());
    assert_eq!(204, response.unwrap().status);
}

#[test]
fn unapply_service_test() {
    let setup = setup_mock_for(
        "/1010/domains/example.com/services/wordpress",
        "unapplyService/success",
        "DELETE",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = "example.com";
    let service = "wordpress";

    let response =
        client
            .services()
            .unapply_service(account_id, String::from(domain), String::from(service));

    assert!(response.is_ok());
    assert_eq!(204, response.unwrap().status);
}
