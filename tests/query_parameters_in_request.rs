use crate::common::setup_mock_for;
use std::collections::HashMap;
mod common;
use dnsimple::dnsimple::{Endpoint, Filters, Paginate, RequestOptions, Sort};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Id {
    #[allow(dead_code)] // Unread but required for test fixture
    pub id: u64,
}

struct IdsEndpoint;

impl Endpoint for IdsEndpoint {
    type Output = Vec<Id>;
}

#[test]
fn can_paginate() {
    let setup = setup_mock_for("/pagination_test?page=2&per_page=2", "pages-2of3", "GET");
    let client = setup.0;
    let options = RequestOptions {
        filters: None,
        sort: None,
        paginate: Some(Paginate {
            per_page: 2,
            page: 2,
        }),
    };

    client
        .get::<IdsEndpoint>("/pagination_test", Some(options))
        .unwrap();
}

#[test]
fn can_filter() {
    let setup = setup_mock_for("/filter_test?name_like=example", "pages-2of3", "GET");
    let client = setup.0;
    let mut filters = HashMap::new();
    filters.insert("name_like".to_string(), "example".to_string());
    let options = RequestOptions {
        filters: Some(Filters { filters }),
        sort: None,
        paginate: None,
    };

    client
        .get::<IdsEndpoint>("/filter_test", Some(options))
        .unwrap();
}

#[test]
fn can_sort() {
    let setup = setup_mock_for("/sort_test?sort=expiration%3Aasc", "pages-2of3", "GET");
    let client = setup.0;
    let options = RequestOptions {
        filters: None,
        sort: Some(Sort {
            sort_by: "expiration:asc".to_string(),
        }),
        paginate: None,
    };

    client
        .get::<IdsEndpoint>("/sort_test", Some(options))
        .unwrap();
}
