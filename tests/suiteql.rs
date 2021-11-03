use httpmock::{Method::POST, Mock, MockServer};
use itertools::Itertools;
use netsuite::{Config, RestApi};
use rstest::rstest;
use std::{ops::RangeInclusive, sync::Once};

fn make_prices(range: RangeInclusive<usize>) -> Vec<Price> {
    let mut prices = Vec::new();
    for p in range {
        prices.push(Price {
            unitprice: p.to_string(),
        })
    }
    prices
}

fn price_bodies(count: usize, limit: usize) -> Vec<Body> {
    let mut bodies = Vec::with_capacity(count / limit + 1);
    let mut num_created = 0_usize;
    for chunk in &(1..=count).chunks(limit) {
        let mut iter = chunk.map(|i| i);
        let start = iter.next().unwrap();
        let range = RangeInclusive::new(start, iter.last().unwrap_or(start));
        let prices = make_prices(range);
        num_created += prices.len();
        let body = Body {
            links: [],
            count: prices.len(),
            has_more: num_created < count,
            total_results: count,
            items: prices,
        };
        bodies.push(body);
    }
    bodies
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Body {
    links: [bool; 0],
    count: usize,
    has_more: bool,
    total_results: usize,
    items: Vec<Price>,
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
struct Price {
    unitprice: String,
}

static INIT: Once = Once::new();

fn ensure_logging() {
    INIT.call_once(|| {
        env_logger::init();
    });
}

fn make_api(server: &MockServer) -> RestApi {
    let config = Config::new("1", "2", "3", "4", "5");
    RestApi::new(config).with_base_url(server.base_url())
}

#[rstest]
fn fetch_values(
    #[values(1, 1001, 2479, 10495)] n_results: usize,
    #[values(1000, 2000)] limit: usize,
) {
    ensure_logging();

    let server = MockServer::start();
    let api = {
        let mut api = make_api(&server);
        api.suiteql.set_default_limit(limit);
        api
    };

    let mocks: Vec<Mock> = price_bodies(n_results, limit)
        .iter()
        .enumerate()
        .map(|(i, body)| {
            server.mock(|when, then| {
                when.method(POST)
                    .path("/query/v1/suiteql")
                    .query_param("limit", &limit.to_string())
                    .query_param("offset", &(i * limit).to_string());
                then.status(200).json_body_obj(body);
            })
        })
        .collect();

    let res = api.suiteql.fetch_values("SELECT * FROM pricing");
    let values = res.unwrap();
    assert_eq!(values.len(), n_results);
    for mock in mocks {
        mock.assert();
    }
    let expected_prices = make_prices(1..=n_results);
    let expected_values: Vec<serde_json::Value> = expected_prices
        .into_iter()
        .map(|p| serde_json::to_value(p).unwrap())
        .collect();
    assert_eq!(values, expected_values);
}

#[rstest]
fn fetch_all(#[values(1, 1001, 2479, 10495)] n_results: usize, #[values(1000, 2000)] limit: usize) {
    ensure_logging();

    let server = MockServer::start();
    let api = {
        let mut api = make_api(&server);
        api.suiteql.set_default_limit(limit);
        api
    };

    let mocks: Vec<Mock> = price_bodies(n_results, limit)
        .iter()
        .enumerate()
        .map(|(i, body)| {
            server.mock(|when, then| {
                when.method(POST)
                    .path("/query/v1/suiteql")
                    .query_param("limit", &limit.to_string())
                    .query_param("offset", &(i * limit).to_string());
                then.status(200).json_body_obj(body);
            })
        })
        .collect();

    let res = api.suiteql.fetch_all::<Price>("SELECT * FROM pricing");
    let prices = res.unwrap();
    assert_eq!(prices.len(), n_results);
    for mock in mocks {
        mock.assert();
    }
    let expected_prices = make_prices(1..=n_results);
    assert_eq!(prices, expected_prices);
}
