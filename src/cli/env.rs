pub enum EnvVar {
    Account,
    ConsumerKey,
    ConsumerSecret,
    TokenId,
    TokenSecret,
}

impl EnvVar {
    pub fn exists(key: &str) -> bool {
        match key {
            "ACCOUNT" => true,
            "CONSUMER_KEY" => true,
            "CONSUMER_SECRET" => true,
            "TOKEN_ID" => true,
            "TOKEN_SECRET" => true,
            _ => false,
        }
    }

    pub fn set(key: &str, val: &str) {
        if EnvVar::exists(key) {
            std::env::set_var(key, val);
        }
    }
}

impl From<EnvVar> for &'static str {
    fn from(var: EnvVar) -> Self {
        match var {
            EnvVar::Account => "ACCOUNT",
            EnvVar::ConsumerKey => "CONSUMER_KEY",
            EnvVar::ConsumerSecret => "CONSUMER_SECRET",
            EnvVar::TokenId => "TOKEN_ID",
            EnvVar::TokenSecret => "TOKEN_SECRET",
        }
    }
}
