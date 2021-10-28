# netsuite-rs

Library for making requests to the NetSuite REST APIs.

Supports both programmatic and CLI usage.

Currently using ureq for HTTP requests, which means this library is not useful for async environments currently. Async will probably be released as a feature flag in the future.

## Beta quality disclaimer

The project's API is still very much in fluctuation. Pin your dependency to the current minor version to avoid breaking changes. From 1.0 and forward we will keep a stable API.

## CLI

This is the easiest way to get started. To find out what you can do with the CLI, just append `--help` or `-h` to the installed `netsuite` command.

### Config file
It's recommended to create an INI config file before using the CLI, to avoid having to provide all OAuth 1.0 details with every command execution.

You can use `netsuite default-ini-path` to find the default INI config location.
```bash
netsuite default-ini-path
```

Write your settings to the file, in this example using [heredoc syntax](https://en.wikipedia.org/wiki/Here_document):
```bash
cat <<EOF >"$(netsuite default-ini-path)"
[sandbox]
account = <account id>_SB1
consumer_key = <64 chars>
consumer_secret = <64 chars>
token_id = <64 chars>
token_secret = <64 chars>
EOF
```

After this you just have to provide the section name to start using the CLI:
```bash
netsuite -s sandbox suiteql 'SELECT * FROM pricing'
```

### Environment variables

As an alternative you can provide environment variables directly. For example:
```bash
export ACCOUNT=<account id>
export CONSUMER_KEY=<64 chars>
export CONSUMER_SECRET=<64 chars>
export TOKEN_ID=<64 chars>
export TOKEN_SECRET=<64 chars>
netsuite suiteql 'SELECT * FROM pricing'
```

## Programmatic access

See example below on how to integrate into your code.

(You can ignore lines prepended with # if you see them, they are there to ensure that provided rust code is correct.)

```rust
use netsuite::{Config, RestApi};

#[derive(Debug, PartialEq, serde::Deserialize)]
struct Price {
    internalid: String,
    unitprice: String,
}

let config = Config::new("123456", "2", "3", "4", "5");
let api = RestApi::new(&config);
# use httpmock::{MockServer, Method::POST};
# let server = MockServer::start();
# let api = RestApi::with_base_url(&config, server.base_url());;
# let mock = server.mock(|when, then| {
#     when.method(POST).path("/query/v1/suiteql");
#     then.status(200).body(r#"{"links":[{"rel":"next","href":"https://123456.suitetalk.api.netsuite.com/services/rest/query/v1/suiteql?limit=2&offset=2"},{"rel":"last","href":"https://123456.suitetalk.api.netsuite.com/services/rest/query/v1/suiteql?limit=2&offset=1998"},{"rel":"self","href":"https://123456.suitetalk.api.netsuite.com/services/rest/query/v1/suiteql?limit=2"}],"count":2,"hasMore":false,"items":[{"links":[],"currency":"1","internalid":"24","item":"24","pricelevel":"15","quantity":"1","saleunit":"1","unitprice":"95.49"},{"links":[],"currency":"1","internalid":"24","item":"24","pricelevel":"21","quantity":"1","saleunit":"1","unitprice":"19.99"}],"offset":0,"totalResults":2000}"#);
# });
let res = api.suiteql.fetch_all::<Price>("SELECT * FROM pricing");
# mock.assert();
assert_eq!(res.unwrap(), [Price { internalid: "24".into(), unitprice: "95.49".into() }, Price { internalid: "24".into(), unitprice: "19.99".into() }]);
```
