use super::CliError;

pub enum EnvVar {
    Account,
    ConsumerKey,
    ConsumerSecret,
    TokenId,
    TokenSecret,
}

impl EnvVar {
    pub fn exists(key: &str) -> bool {
        matches!(
            key,
            "ACCOUNT" | "CONSUMER_KEY" | "CONSUMER_SECRET" | "TOKEN_ID" | "TOKEN_SECRET"
        )
    }

    pub fn set<'a>(key: &'a str, val: &'a str) -> Result<&'a str, CliError> {
        if EnvVar::exists(key) {
            std::env::set_var(key, val);
            Ok(val)
        } else {
            Err(CliError::UnknownEnvironmentVariable(key.to_string()))
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
