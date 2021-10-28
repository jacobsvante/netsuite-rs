use httpmock::{Method::POST, MockServer};
use netsuite::{Config, RestApi};
use std::sync::Once;

#[derive(Debug, PartialEq, serde::Deserialize)]
struct Price {
    unitprice: String,
}

static INIT: Once = Once::new();

fn ensure_logging() {
    INIT.call_once(|| {
        env_logger::init();
    });
}

fn make_config<'a>() -> Config<'a> {
    Config::new("1", "2", "3", "4", "5")
}

fn make_api<'a>(config: &'a Config, server: &MockServer) -> RestApi<'a> {
    RestApi::with_base_url(&config, server.base_url())
}

#[test]
fn raw_no_params() {
    ensure_logging();
    let server = MockServer::start();
    let config = make_config();
    let api = make_api(&config, &server);
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/query/v1/suiteql")
            .query_param("limit", "1000")
            .query_param("offset", "0");
        then.status(200).body("test");
    });
    let res = api.suiteql.raw("SELECT * FROM pricing", 1000, 0);
    mock.assert();
    assert_eq!(res.unwrap().body(), "test");
}

#[test]
fn raw_limit_param() {
    ensure_logging();
    let server = MockServer::start();
    let config = make_config();
    let api = make_api(&config, &server);
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/query/v1/suiteql")
            .query_param("limit", "2")
            .query_param("offset", "0");
        then.status(200).body("test");
    });
    let res = api.suiteql.raw("SELECT * FROM pricing", 2, 0);
    mock.assert();
    assert_eq!(res.unwrap().body(), "test");
}

#[test]
fn no_params() {
    ensure_logging();
    let server = MockServer::start();
    let config = make_config();
    let api = {
        let mut api = make_api(&config, &server);
        api.suiteql.set_limit(2);
        api
    };
    let mock1 = server.mock(|when, then| {
        when.method(POST).path("/query/v1/suiteql").query_param("offset", "0");
        then.status(200).body(r#"{"links": [], "count": 2, "hasMore": true, "totalResults": 4, "items": [{"unitprice": "1"}, {"unitprice": "2"}]}"#);
    });
    let mock2 = server.mock(|when, then| {
        when.method(POST).path("/query/v1/suiteql").query_param("offset", "2");
        then.status(200).body(r#"{"links": [], "count": 2, "hasMore": false, "totalResults": 4, "items": [{"unitprice": "3"}, {"unitprice": "4"}]}"#);
    });

    let res = api.suiteql.fetch_all::<Price>("SELECT * FROM pricing");
    let prices = res.unwrap();
    mock1.assert();
    mock2.assert();
    assert_eq!(
        prices,
        [
            Price {
                unitprice: "1".into()
            },
            Price {
                unitprice: "2".into()
            },
            Price {
                unitprice: "3".into()
            },
            Price {
                unitprice: "4".into()
            }
        ]
    );
}
