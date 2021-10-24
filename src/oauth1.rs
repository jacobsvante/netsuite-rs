use percent_encoding::{AsciiSet, NON_ALPHANUMERIC};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use ring::hmac::{self, HMAC_SHA1_FOR_LEGACY_USE_ONLY};
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub key: &'a str,
    pub secret: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(key: &'a str, secret: &'a str) -> Self {
        Token { key, secret }
    }
}

pub fn authorize(
    method: &str,
    uri: &str,
    consumer: &Token,
    token: Option<&Token>,
    params: Option<HashMap<&str, &str>>,
    realm: Option<&str>,
) -> String {
    let mut params = params.unwrap_or_else(HashMap::new);
    // duration_since might fail if the system clock is set to before the UNIX epoch.
    // Handling this by just setting timestamp to 0 in that case
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_or(0, |v| v.as_secs())
        .to_string();

    let mut rng = thread_rng();
    let nonce: String = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(32)
        .collect();

    params.insert("oauth_consumer_key", consumer.key);
    params.insert("oauth_nonce", &nonce);
    params.insert("oauth_signature_method", "HMAC-SHA1");
    params.insert("oauth_timestamp", &timestamp);
    params.insert("oauth_version", "1.0");

    if let Some(tk) = token {
        params.insert("oauth_token", tk.key);
    }

    let signature = gen_signature(
        method,
        uri,
        &to_query(&params),
        consumer.secret,
        token.map(|t| t.secret),
    );

    params.insert("oauth_signature", &signature);

    let mut pairs = params
        .iter()
        .filter(|&(k, _)| k.starts_with("oauth_"))
        .map(|(k, v)| format!("{}=\"{}\"", k, encode(v)))
        .collect::<Vec<_>>();

    pairs.sort();

    if let Some(realm) = realm {
        pairs.insert(0, format!("realm=\"{}\"", realm));
    }

    format!("OAuth {}", pairs.join(", "))
}

#[derive(Copy, Clone)]
struct StrictEncodeSet;

// Encode all but the unreserved characters defined in
// RFC 3986, section 2.3. "Unreserved Characters"
// https://tools.ietf.org/html/rfc3986#page-12
//
// This is required by
// OAuth Core 1.0, section 5.1. "Parameter Encoding"
// https://oauth.net/core/1.0/#encoding_parameters
static STRICT_ENCODE_SET: AsciiSet = NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

fn encode(s: &str) -> String {
    percent_encoding::percent_encode(s.as_bytes(), &STRICT_ENCODE_SET).collect()
}

fn to_query(params: &HashMap<&str, &str>) -> String {
    let mut pairs: Vec<_> = params
        .iter()
        .map(|(k, v)| format!("{}={}", encode(k), encode(v)))
        .collect();

    pairs.sort();
    pairs.join("&")
}

fn gen_signature(
    method: &str,
    uri: &str,
    query: &str,
    consumer_secret: &str,
    token_secret: Option<&str>,
) -> String {
    let base = format!("{}&{}&{}", encode(method), encode(uri), encode(query));

    let key = format!(
        "{}&{}",
        encode(consumer_secret),
        encode(token_secret.unwrap_or(""))
    );

    let s_key = hmac::Key::new(HMAC_SHA1_FOR_LEGACY_USE_ONLY, key.as_ref());
    let signature = hmac::sign(&s_key, base.as_bytes());

    base64::encode(signature.as_ref())
}
