use httpmock::{Method::GET, MockServer};
use netsuite::{oauth1::Algorithm, Config, RestApi};
use std::sync::Once;

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

#[test]
fn defaults_to_hmac_sha256() {
    ensure_logging();
    let server = MockServer::start();
    let api = make_api(&server);
    let mock = server.mock(|when, then| {
        when.method(GET).path("/record/v1/invoice").matches(|r| {
            let vec = r.headers.clone().unwrap();
            let (_, v) = vec.iter().find(|(k, _)| k == "authorization").unwrap();
            assert!(v.contains("oauth_signature_method=\"HMAC-SHA256\""));
            true
        });
        then.status(200).body("test");
    });
    let res = api.get_raw("/record/v1/invoice", None, None);
    mock.assert();
    assert_eq!(res.unwrap().body(), "test");
}

#[test]
fn use_hmac_sha1() {
    ensure_logging();
    let server = MockServer::start();
    let api = make_api(&server).with_algorithm(Algorithm::Sha1);
    let mock = server.mock(|when, then| {
        when.method(GET).path("/record/v1/invoice").matches(|r| {
            let vec = r.headers.clone().unwrap();
            let (_, v) = vec.iter().find(|(k, _)| k == "authorization").unwrap();
            assert!(v.contains("oauth_signature_method=\"HMAC-SHA1\""));
            true
        });
        then.status(200).body("test");
    });
    let res = api.get_raw("/record/v1/invoice", None, None);
    mock.assert();
    assert_eq!(res.unwrap().body(), "test");
}
