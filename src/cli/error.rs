#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("INI path could not be found")]
    MissingIniPath,
}
