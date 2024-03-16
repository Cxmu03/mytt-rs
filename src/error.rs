use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not authenticate with the given credentials")]
    AuthenticationError(),
    #[error("Failed to make a request")]
    ClientError(#[from] ureq::Error)
}