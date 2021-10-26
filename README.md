# netsuite-rs

Library for making requests to the NetSuite REST APIs.

Supports both programmatic and CLI usage.

Currently using ureq for HTTP requests, which means this library is not useful for async environments currently. Async will probably be released as a feature flag in the future.

## Programmatic examples

```rust
# use httpmock::{MockServer, Method::POST};
use netsuite::{Config, RestApi};

#[derive(Debug, PartialEq, serde::Deserialize)]
struct Price {
    internalid: String,
    unitprice: String,
}

# let server = MockServer::start();
let config = Config::new("123456", "2", "3", "4", "5");
let api = RestApi::new(&config);
# let api = RestApi::with_base_url(&config, server.base_url());;
# let mock = server.mock(|when, then| {
#     when.method(POST).path("/suiteql");
#     then.status(200).body(r#"{"links":[{"rel":"next","href":"https://123456.suitetalk.api.netsuite.com/services/rest/query/v1/suiteql?limit=2&offset=2"},{"rel":"last","href":"https://123456.suitetalk.api.netsuite.com/services/rest/query/v1/suiteql?limit=2&offset=1998"},{"rel":"self","href":"https://123456.suitetalk.api.netsuite.com/services/rest/query/v1/suiteql?limit=2"}],"count":2,"hasMore":false,"items":[{"links":[],"currency":"1","internalid":"24","item":"24","pricelevel":"15","quantity":"1","saleunit":"1","unitprice":"95.49"},{"links":[],"currency":"1","internalid":"24","item":"24","pricelevel":"21","quantity":"1","saleunit":"1","unitprice":"19.99"}],"offset":0,"totalResults":2000}"#);
# });
let res = api.suiteql.fetch_all::<Price>("SELECT * FROM pricing");
# mock.assert();
assert_eq!(res.unwrap(), [Price { internalid: "24".into(), unitprice: "95.49".into() }, Price { internalid: "24".into(), unitprice: "19.99".into() }]);
```

## CLI examples

Get all prices via SuiteQL.

```bash
export ACCOUNT=<6 chars>;
export CONSUMER_KEY=<64 chars>;
export CONSUMER_SECRET=<64 chars>;
export TOKEN_ID=<64 chars>;
export TOKEN_SECRET=<64 chars>;
netsuite suiteql 'SELECT * FROM pricing'
```
