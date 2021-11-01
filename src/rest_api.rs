use crate::oauth1::Algorithm;
use crate::response::Response;

use super::config::Config;
use super::error::Error;
use super::metadata_api::MetadataApi;
use super::params::Params;
use super::requester::Requester;
use super::suiteql::SuiteQl;

static DEFAULT_BASE_URL: &str = "https://{}.suitetalk.api.netsuite.com/services/rest";

pub struct RestApi {
    requester: Requester,
    pub suiteql: SuiteQl,
    pub metadata: MetadataApi,
}

impl RestApi {
    pub fn new(config: Config) -> Self {
        let base_url = Self::default_base_url(&config);
        let requester = Requester::new(config, base_url);
        let suiteql = SuiteQl::new(requester.clone());
        let metadata = MetadataApi::new(requester.clone());
        Self {
            requester,
            suiteql,
            metadata,
        }
    }

    fn default_base_url(config: &Config) -> String {
        let host_part = config.account.replace("_", "-").to_lowercase();
        DEFAULT_BASE_URL.replace("{}", &host_part)
    }

    pub fn with_base_url(self, base_url: String) -> Self {
        let requester = self.requester.with_base_url(base_url);
        let suiteql = SuiteQl::new(requester.clone());
        let metadata = MetadataApi::new(requester.clone());
        Self {
            requester,
            suiteql,
            metadata,
        }
    }

    pub fn with_algorithm(self, algorithm: Algorithm) -> Self {
        let requester = self.requester.with_algorithm(algorithm);
        let suiteql = SuiteQl::new(requester.clone());
        let metadata = MetadataApi::new(requester.clone());
        Self {
            requester,
            suiteql,
            metadata,
        }
    }

    pub fn get_raw(
        &self,
        endpoint: &str,
        params: Option<Params>,
        headers: Option<Params>,
    ) -> Result<Response, Error> {
        self.requester
            .request(::http::Method::GET, endpoint, params, headers, None)
    }

    pub fn post_raw(
        &self,
        endpoint: &str,
        params: Option<Params>,
        headers: Option<Params>,
        payload: Option<&str>,
    ) -> Result<Response, Error> {
        self.requester
            .request(::http::Method::POST, endpoint, params, headers, payload)
    }

    pub fn put_raw(
        &self,
        endpoint: &str,
        params: Option<Params>,
        headers: Option<Params>,
        payload: Option<&str>,
    ) -> Result<Response, Error> {
        self.requester
            .request(::http::Method::PUT, endpoint, params, headers, payload)
    }

    pub fn patch_raw(
        &self,
        endpoint: &str,
        params: Option<Params>,
        headers: Option<Params>,
        payload: Option<&str>,
    ) -> Result<Response, Error> {
        self.requester
            .request(::http::Method::PATCH, endpoint, params, headers, payload)
    }

    pub fn delete_raw(
        &self,
        endpoint: &str,
        params: Option<Params>,
        headers: Option<Params>,
    ) -> Result<Response, Error> {
        self.requester
            .request(::http::Method::DELETE, endpoint, params, headers, None)
    }
}
