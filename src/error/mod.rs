use std::fmt::{Display, Formatter};

pub type IGDBResult<T> = Result<T, IGDBError>;

#[derive(Debug)]
pub enum IGDBError {
    Http(reqwest::Error),
    Serialization(serde_json::Error),
    EnvVar(std::env::VarError),
}

impl std::error::Error for IGDBError {}

impl Display for IGDBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IGDBError::Http(error) => write!(f, "{}", error),
            IGDBError::Serialization(error) => write!(f, "{}", error),
            IGDBError::EnvVar(error) => write!(f, "{}", error),
        }
    }
}

impl From<reqwest::Error> for IGDBError {
    fn from(error: reqwest::Error) -> Self {
        IGDBError::Http(error)
    }
}

impl From<serde_json::error::Error> for IGDBError {
    fn from(error: serde_json::error::Error) -> Self {
        IGDBError::Serialization(error)
    }
}

impl From<std::env::VarError> for IGDBError {
    fn from(error: std::env::VarError) -> Self {
        IGDBError::EnvVar(error)
    }
}
