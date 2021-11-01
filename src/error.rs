#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed HTTP request with status: {0}. Body: {1}")]
    HttpRequestError(u16, String),
    #[error("{0}")]
    HttpTransportError(String),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Serde error")]
    SerializationError(#[from] serde_json::Error),
}
