use crate::oauth1;

pub struct Config<'a> {
    pub account_id: &'a str,
    pub consumer: oauth1::Token<'a>,
    pub token: oauth1::Token<'a>,
}

impl<'a> Config<'a> {
    pub fn new(
        account_id: &'a str,
        consumer_key: &'a str,
        consumer_secret: &'a str,
        token_key: &'a str,
        token_secret: &'a str,
    ) -> Self {
        Self {
            account_id,
            consumer: oauth1::Token::new(consumer_key, consumer_secret),
            token: oauth1::Token::new(token_key, token_secret),
        }
    }
}
