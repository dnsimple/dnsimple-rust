use crate::common::setup_mock_for;
use dnsimple::dnsimple::{Filters, Paginate, RequestOptions, Sort};
use std::collections::HashMap;
mod common;

#[tokio::test]
async fn query_test() {
    let setup = setup_mock_for("/1/dns_analytics", "dnsAnalytics/success", "GET").await;
    let client = setup.0;

    let response = client.dns_analytics().query(1, None).await.unwrap();
    let rows = response.data.unwrap();

    assert_eq!(200, response.status);
    assert_eq!(12, rows.len());

    let first = rows.first().unwrap();
    assert_eq!(Some(String::from("bar.com")), first.zone_name);
    assert_eq!(Some(String::from("2023-12-08")), first.date);
    assert_eq!(Some(1200), first.volume);

    let last = rows.last().unwrap();
    assert_eq!(Some(String::from("foo.com")), last.zone_name);
    assert_eq!(Some(String::from("2024-01-08")), last.date);
    assert_eq!(Some(1200), last.volume);

    let pagination = response.pagination.unwrap();
    assert_eq!(0, pagination.current_page);
    assert_eq!(100, pagination.per_page);
    assert_eq!(93, pagination.total_entries);
    assert_eq!(1, pagination.total_pages);

    let query = response.query.unwrap();
    assert_eq!(1, query.account_id);
    assert_eq!(Some(String::from("2023-12-08")), query.start_date);
    assert_eq!(Some(String::from("2024-01-08")), query.end_date);
    assert_eq!("zone_name:asc,date:asc", query.sort);
    assert_eq!(0, query.page);
    assert_eq!(100, query.per_page);
    assert_eq!(Some(String::from("zone_name,date")), query.groupings);
}

#[tokio::test]
async fn query_with_options_test() {
    let setup = setup_mock_for(
        "/1/dns_analytics?page=2&per_page=10&groupings=zone_name%2Cdate&sort=volume%3Adesc",
        "dnsAnalytics/success",
        "GET",
    )
    .await;
    let client = setup.0;
    let mut filters = HashMap::new();
    filters.insert(String::from("groupings"), String::from("zone_name,date"));
    let options = RequestOptions {
        filters: Some(Filters::new(filters)),
        sort: Some(Sort::new(String::from("volume:desc"))),
        paginate: Some(Paginate {
            per_page: 10,
            page: 2,
        }),
    };

    let response = client
        .dns_analytics()
        .query(1, Some(options))
        .await
        .unwrap();

    assert_eq!(200, response.status);
}
