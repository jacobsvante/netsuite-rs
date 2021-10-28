#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("Unable to determine INI path")]
    MissingIniPath,
    #[error("Unable to determine INI section")]
    MissingIniSection,
    #[error("Unknown environment variable: {0}")]
    UnknownEnvironmentVariable(String),
    #[error("Parameter format invalid")]
    BadParam,
}
