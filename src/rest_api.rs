use crate::config::Config;
use crate::requester::Requester;
use crate::suiteql::SuiteQl;

static DEFAULT_BASE_URL: &str = "https://{}.suitetalk.api.netsuite.com/services/rest/query/v1";

pub struct RestApi<'a> {
    pub suiteql: SuiteQl<'a>,
}

impl<'a> RestApi<'a> {
    pub fn new(config: &'a Config) -> Self {
        let base_url = DEFAULT_BASE_URL.replace("{}", config.account);
        let requester = Requester::new(config, base_url);
        let suiteql = SuiteQl::new(requester);
        Self { suiteql }
    }

    pub fn with_base_url(config: &'a Config, base_url: String) -> Self {
        let requester = Requester::new(config, base_url);
        let suiteql = SuiteQl::new(requester);
        Self { suiteql }
    }
}
