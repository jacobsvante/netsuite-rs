use crate::error::Error;
use crate::params::Params;
use crate::requester::Requester;
use crate::response::Response;
use http::Method;

pub struct SuiteQl {
    requester: Requester,
    limit: usize,
}

impl SuiteQl {
    pub fn new(requester: Requester) -> Self {
        Self {
            requester,
            limit: 1000,
        }
    }

    pub fn set_limit(&mut self, limit: usize) {
        self.limit = limit;
    }

    pub fn raw(&self, query: &str, limit: usize, offset: usize) -> Result<Response, Error> {
        let mut params = Params::new();
        params.push("limit", limit);
        params.push("offset", offset);
        let mut headers = Params::new();
        let payload = SuiteQlPayload { q: query };
        let payload = serde_json::to_string(&payload)?;
        headers.push("Prefer", "transient");
        self.requester.request(
            Method::POST,
            "query/v1/suiteql",
            Some(params),
            Some(headers),
            Some(&payload),
        )
    }

    pub fn fetch_all<T: serde::de::DeserializeOwned>(&self, query: &str) -> Result<Vec<T>, Error> {
        let mut collected = Vec::new();
        for i in 0.. {
            let res = self.raw(query, self.limit, self.limit * i)?;
            let res: SuiteQlResponse = serde_json::from_str(res.body())?;
            let parsed: Vec<T> = serde_json::from_value(res.items)?;
            collected.extend(parsed);
            if !res.has_more {
                break;
            }
        }
        Ok(collected)
    }
}

#[derive(Debug, serde::Deserialize)]
struct Link {
    rel: String,
    href: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SuiteQlResponse {
    links: Vec<Link>,
    count: usize,
    has_more: bool,
    total_results: usize,
    items: serde_json::value::Value,
}

#[derive(serde::Serialize)]
struct SuiteQlPayload<'a> {
    q: &'a str,
}
