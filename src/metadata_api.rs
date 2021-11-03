use crate::error::Error;
use crate::params::Params;
use crate::requester::Requester;
use crate::response::Response;
use http::Method;

#[derive(Debug, Clone)]
pub struct MetadataApi {
    requester: Requester,
}

impl MetadataApi {
    pub fn new(requester: Requester) -> Self {
        Self { requester }
    }

    pub fn raw(
        &self,
        endpoint: &str,
        params: Option<Params>,
        headers: Option<Params>,
    ) -> Result<Response, Error> {
        self.requester
            .request(Method::GET, endpoint, params, headers, None)
    }

    pub fn jsonschema(&self, record_type: &str) -> Result<Response, Error> {
        let endpoint = format!("/record/v1/metadata-catalog/{}", record_type);
        let mut headers = Params::new();
        headers.push("Accept", "application/schema+json");
        self.raw(&endpoint, None, Some(headers))
    }

    pub fn openapi(&self, record_types: &[String]) -> Result<Response, Error> {
        let mut headers = Params::new();
        headers.push("Accept", "application/swagger+json");
        let mut params = Params::new();
        if !record_types.is_empty() {
            params.push("select", record_types.join(","))
        }
        self.raw("/record/v1/metadata-catalog", Some(params), Some(headers))
    }
}
