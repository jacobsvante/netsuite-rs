use crate::oauth1;

#[derive(Clone, Debug)]
pub struct Config {
    pub account: String,
    pub consumer: oauth1::Token,
    pub token: oauth1::Token,
}

impl Config {
    pub fn new<T: ToString>(
        account: T,
        consumer_key: T,
        consumer_secret: T,
        token_id: T,
        token_secret: T,
    ) -> Self {
        Self {
            account: account.to_string(),
            consumer: oauth1::Token::new(consumer_key.to_string(), consumer_secret.to_string()),
            token: oauth1::Token::new(token_id.to_string(), token_secret.to_string()),
        }
    }
}
