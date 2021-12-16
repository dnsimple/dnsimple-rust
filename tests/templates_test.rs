use crate::common::setup_mock_for;
use dnsimple::dnsimple::templates::{TemplatePayload, TemplateRecordPayload};
mod common;

#[test]
fn list_templates_test() {
    let setup = setup_mock_for("/1010/templates", "listTemplates/success", "GET");
    let client = setup.0;
    let account_id = 1010;

    let templates = client
        .templates()
        .list_templates(account_id, None)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(2, templates.len());
}

#[test]
fn create_template_test() {
    let setup = setup_mock_for("/1010/templates", "createTemplate/created", "POST");
    let client = setup.0;
    let account_id = 1010;
    let payload = TemplatePayload {
        name: String::from("Beta"),
        sid: String::from("beta"),
        description: Some(String::from("A beta template.")),
    };

    let template = client
        .templates()
        .create_template(account_id, payload)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(1, template.id);
    assert_eq!(1010, template.account_id);
    assert_eq!("Beta", template.name);
    assert_eq!("beta", template.sid);
    assert_eq!("A beta template.", template.description);
    assert_eq!("2016-03-24T11:09:16Z", template.created_at);
    assert_eq!("2016-03-24T11:09:16Z", template.updated_at);
}

#[test]
fn get_template_test() {
    let setup = setup_mock_for("/1010/templates/alpha", "getTemplate/success", "GET");
    let client = setup.0;
    let account_id = 1010;
    let template = String::from("alpha");

    let template = client
        .templates()
        .get_template(account_id, template)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(1, template.id);
    assert_eq!(1010, template.account_id);
    assert_eq!("Alpha", template.name);
    assert_eq!("alpha", template.sid);
    assert_eq!("An alpha template.", template.description);
    assert_eq!("2016-03-22T11:08:58Z", template.created_at);
    assert_eq!("2016-03-22T11:08:58Z", template.updated_at);
}

#[test]
fn update_template_test() {
    let setup = setup_mock_for("/1010/templates/beta", "updateTemplate/success", "PATCH");
    let client = setup.0;
    let account_id = 1010;
    let template_id = String::from("beta");

    let payload = TemplatePayload {
        name: String::from("Alpha"),
        sid: String::from("alpha"),
        description: Some(String::from("An alpha template.")),
    };

    let template = client
        .templates()
        .update_template(account_id, template_id, payload)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(1, template.id);
    assert_eq!(1010, template.account_id);
    assert_eq!("Alpha", template.name);
    assert_eq!("alpha", template.sid);
    assert_eq!("An alpha template.", template.description);
    assert_eq!("2016-03-22T11:08:58Z", template.created_at);
    assert_eq!("2016-03-22T11:08:58Z", template.updated_at);
}

#[test]
fn delete_template() {
    let setup = setup_mock_for("/1010/templates/beta", "deleteTemplate/success", "DELETE");
    let client = setup.0;
    let account_id = 1010;
    let template_id = String::from("beta");

    let response = client.templates().delete_template(account_id, template_id);

    assert_eq!(204, response.status);
}

#[test]
fn list_template_records() {
    let setup = setup_mock_for(
        "/1010/templates/beta/records",
        "listTemplateRecords/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1010;
    let template = String::from("beta");

    let records = client
        .templates()
        .list_template_records(account_id, template, None)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(2, records.len());
}

#[test]
fn create_template_record() {
    let setup = setup_mock_for(
        "/1010/templates/beta/records",
        "createTemplateRecord/created",
        "POST",
    );
    let client = setup.0;
    let account_id = 1010;
    let template = String::from("beta");
    let payload = TemplateRecordPayload {
        name: String::from("Beta"),
        record_type: String::from("MX"),
        content: String::from("mx.example.com"),
        ttl: None,
        priority: None,
    };

    let record = client
        .templates()
        .create_template_record(account_id, template, payload)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(300, record.id);
    assert_eq!(268, record.template_id);
    assert_eq!("", record.name);
    assert_eq!("mx.example.com", record.content);
    assert_eq!(600, record.ttl);
    assert_eq!(Some(10), record.priority);
    assert_eq!("MX", record.record_type);
    assert_eq!("2016-05-03T07:51:33Z", record.created_at);
    assert_eq!("2016-05-03T07:51:33Z", record.updated_at);
}

#[test]
fn get_template_record() {
    let setup = setup_mock_for(
        "/1010/templates/beta/records/301",
        "getTemplateRecord/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1010;
    let template = String::from("beta");
    let record_id = 301;

    let record = client
        .templates()
        .get_template_record(account_id, template, record_id)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(301, record.id);
    assert_eq!(268, record.template_id);
    assert_eq!("", record.name);
    assert_eq!("mx.example.com", record.content);
    assert_eq!(600, record.ttl);
    assert_eq!(Some(10), record.priority);
    assert_eq!("MX", record.record_type);
    assert_eq!("2016-05-03T08:03:26Z", record.created_at);
    assert_eq!("2016-05-03T08:03:26Z", record.updated_at);
}

#[test]
fn delete_template_record() {
    let setup = setup_mock_for(
        "/1010/templates/beta/records/301",
        "deleteTemplateRecord/success",
        "DELETE",
    );
    let client = setup.0;
    let account_id = 1010;
    let template = String::from("beta");
    let record_id = 301;

    let response = client
        .templates()
        .delete_template_record(account_id, template, record_id);

    assert_eq!(204, response.status);
}

#[test]
fn apply_template() {
    let setup = setup_mock_for(
        "/1010/domains/example.com/templates/301",
        "applyTemplate/success",
        "POST",
    );
    let client = setup.0;
    let account_id = 1010;
    let domain = String::from("example.com");
    let template = String::from("301");

    let response = client
        .templates()
        .apply_template(account_id, domain, template);

    assert_eq!(204, response.status);
}
