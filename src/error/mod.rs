use std::fmt::{Display, Formatter};

pub type IGDBResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Serialization(serde_json::Error),
    EnvVar(std::env::VarError),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(error) => write!(f, "{}", error),
            Error::Serialization(error) => write!(f, "{}", error),
            Error::EnvVar(error) => write!(f, "{}", error),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Http(error)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        Error::Serialization(error)
    }
}

impl From<std::env::VarError> for Error {
    fn from(error: std::env::VarError) -> Self {
        Error::EnvVar(error)
    }
}
