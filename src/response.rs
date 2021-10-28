use std::collections::HashMap;

use http::StatusCode;

#[derive(Debug)]
pub struct Response {
    status: StatusCode,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    pub fn new(status: StatusCode, headers: HashMap<String, String>, body: String) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let headers = self
            .headers()
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\n");
        write!(
            f,
            "{}\n{}\n[Body: {} bytes]",
            self.status(),
            headers,
            self.body().len()
        )
    }
}
