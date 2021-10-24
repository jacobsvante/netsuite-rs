use crate::config::Config;
use crate::oauth1;
use crate::params::Params;
use crate::Error;
use http::Method;
use log::info;

pub struct Requester<'a> {
    config: &'a Config<'a>,
    base_url: String,
}

impl<'a> Requester<'a> {
    pub fn new(config: &'a Config, base_url: String) -> Self {
        Self { config, base_url }
    }

    fn make_url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url, endpoint)
    }

    fn auth_header(&self, method: Method, url: &str, params: &Option<Params>) -> String {
        oauth1::authorize(
            method.as_str(),
            url,
            &self.config.consumer,
            Some(&self.config.token),
            params.clone().map(|p| p.into()),
            Some(self.config.account_id),
        )
    }

    pub fn request(
        &self,
        method: Method,
        endpoint: &str,
        params: Option<Params>,
        headers: Option<Params>,
        payload: Option<&str>,
    ) -> Result<String, Error> {
        let url = self.make_url(endpoint);
        let auth = self.auth_header(method, &url, &params);

        let mut req = ureq::post(&url)
            .set("Authorization", &auth)
            .set("Content-Type", "application/json");

        if let Some(params) = params {
            for (k, v) in params.get() {
                req = req.query(k, v);
            }
        }

        if let Some(headers) = headers {
            for (k, v) in headers.get() {
                req = req.set(k, v);
            }
        }

        let resp = if let Some(payload) = payload {
            info!("Payload: {}", payload);
            req.send_string(payload)
        } else {
            req.call()
        };

        let resp = match resp {
            Ok(resp) => resp,
            Err(ureq::Error::Status(code, resp)) => {
                return Err(Error::HttpRequestError(
                    code,
                    resp.into_string().unwrap_or_default(),
                ));
            }
            Err(ureq::Error::Transport(transport)) => {
                return Err(Error::HttpTransportError(transport.to_string()));
            }
        };
        let body = resp.into_string()?;
        Ok(body)
    }
}
