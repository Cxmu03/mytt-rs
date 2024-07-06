use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not authenticate with the given credentials")]
    AuthenticationError(String),
    #[error("Failed to make a request")]
    ClientError(#[from] ureq::Error),
    #[error("Failed to validate data")]
    ValidationError(String),
    #[error("Failed to parse data")]
    ParsingError(String),
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
}
