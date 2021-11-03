use core::num::NonZeroU8;
use std::collections::VecDeque;
use std::thread;

use http::Method;
use itertools::Itertools;
use log::debug;
use serde_json::Value;

use crate::error::Error;
use crate::params::Params;
use crate::requester::Requester;

#[derive(Debug, Clone)]
pub struct SuiteQl {
    requester: Requester,
    default_limit: usize,
    threads: NonZeroU8,
}

impl SuiteQl {
    pub fn new(requester: Requester) -> Self {
        Self {
            requester,
            default_limit: 1000,
            threads: NonZeroU8::new(10).unwrap(),
        }
    }

    pub fn set_default_limit(&mut self, default_limit: usize) {
        self.default_limit = default_limit;
    }

    pub fn set_threads(&mut self, threads: NonZeroU8) {
        self.threads = threads;
    }

    pub fn raw(&self, query: &str, limit: usize, offset: usize) -> Result<SuiteQlResponse, Error> {
        let mut params = Params::new();
        params.push("limit", limit);
        params.push("offset", offset);
        let mut headers = Params::new();
        let payload = SuiteQlPayload { q: query };
        let payload = serde_json::to_string(&payload)?;
        headers.push("Prefer", "transient");
        let res = self.requester.request(
            Method::POST,
            "query/v1/suiteql",
            Some(params),
            Some(headers),
            Some(&payload),
        )?;
        Ok(serde_json::from_str::<SuiteQlResponse>(res.body())?)
    }

    pub fn fetch_values(&self, query: &str) -> Result<Vec<Value>, Error> {
        let limit = self.default_limit;

        let res = self.raw(query, limit, 0)?;
        let total_results = res.total_results;
        let mut collected = res.items;

        if limit >= total_results {
            return Ok(collected);
        }

        let additional_requests =
            (total_results / limit) - 1 + (if total_results % limit > 0 { 1 } else { 0 });

        let num_threads: u8 = self.threads.into();
        let mut handles = VecDeque::with_capacity(num_threads.into());

        debug!("Fetched initial SuiteQL result set. {} total results. Now doing {} additional requests, on {} threads", total_results, additional_requests, num_threads);

        for req_no_chunk in &(1..=additional_requests).chunks(num_threads.into()) {
            for req_no in req_no_chunk {
                let offset = limit * req_no;
                let api = self.clone();
                let q = query.to_string();
                let builder = thread::Builder::new().name(format!("thread-req-no-{}", req_no));
                let handle = builder.spawn(move || {
                    let res = api.raw(&q, limit, offset);
                    debug!("Retrieved response #{}/{}", req_no, additional_requests);
                    res
                })?;
                handles.push_back(handle);
            }

            while let Some(handle) = handles.pop_front() {
                let mut res = handle.join().unwrap()?;
                collected.append(&mut res.items);
            }
            debug_assert_eq!(handles.len(), 0);
        }
        Ok(collected)
    }

    pub fn fetch_all<T: serde::de::DeserializeOwned>(&self, query: &str) -> Result<Vec<T>, Error> {
        let values = self.fetch_values(query)?;
        let mut typed: Vec<T> = Vec::with_capacity(values.len());
        for value in values {
            typed.push(serde_json::from_value(value)?)
        }
        Ok(typed)
    }
}

#[derive(Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
struct Link {
    rel: String,
    href: String,
}

#[derive(Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuiteQlResponse {
    links: Vec<Link>,
    count: usize,
    has_more: bool,
    total_results: usize,
    items: Vec<Value>,
}

impl SuiteQlResponse {
    pub fn into_typed_items<T: serde::de::DeserializeOwned>(self) -> Result<Vec<T>, Error> {
        let mut vec: Vec<T> = Vec::with_capacity(self.items.len());
        for v in self.items {
            vec.push(serde_json::from_value(v)?);
        }
        Ok(vec)
    }
}

#[derive(serde::Serialize)]
struct SuiteQlPayload<'a> {
    q: &'a str,
}
