use percent_encoding::{AsciiSet, NON_ALPHANUMERIC};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use ring::hmac;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy)]
pub enum Algorithm {
    Sha1,
    Sha256,
}

impl Algorithm {
    fn ring_algorithm(self) -> hmac::Algorithm {
        match self {
            Self::Sha1 => hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
            Self::Sha256 => hmac::HMAC_SHA256,
        }
    }

    fn id(self) -> &'static str {
        match self {
            Self::Sha1 => "HMAC-SHA1",
            Self::Sha256 => "HMAC-SHA256",
        }
    }
}

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
    params: Option<Vec<(String, String)>>,
    realm: Option<&str>,
    algorithm: Algorithm,
) -> String {
    let mut params = params.unwrap_or_else(Vec::new);
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

    params.push(("oauth_nonce".into(), nonce));
    params.push(("oauth_timestamp".into(), timestamp));
    params.push(("oauth_version".into(), "1.0".into()));
    params.push(("oauth_signature_method".into(), algorithm.id().to_owned()));
    params.push(("oauth_consumer_key".into(), consumer.key.to_owned()));

    if let Some(tk) = token {
        params.push(("oauth_token".into(), tk.key.to_owned()));
    }

    let signature = gen_signature(
        algorithm,
        method,
        uri,
        &to_query(&params),
        consumer.secret,
        token.map(|t| t.secret),
    );

    params.push(("oauth_signature".into(), signature));

    let mut pairs = params
        .iter()
        .filter(|&(k, _)| k.starts_with("oauth_"))
        .map(|(k, v)| format!("{}=\"{}\"", k, encode(v)))
        .collect::<Vec<_>>();

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

fn to_query(params: &Vec<(String, String)>) -> String {
    let mut pairs: Vec<_> = params
        .iter()
        .map(|(k, v)| format!("{}={}", encode(k), encode(v)))
        .collect();
    pairs.sort();
    pairs.join("&")
}

fn gen_signature(
    algorithm: Algorithm,
    method: &str,
    uri: &str,
    query: &str,
    consumer_secret: &str,
    token_secret: Option<&str>,
) -> String {
    let key = [encode(consumer_secret), encode(token_secret.unwrap_or(""))].join("&");
    let base = format!("{}&{}&{}", encode(method), encode(uri), encode(query));

    let algo = algorithm.ring_algorithm();
    let s_key = hmac::Key::new(algo, key.as_ref());
    let signature = hmac::sign(&s_key, base.as_bytes());

    base64::encode(signature.as_ref())
}
