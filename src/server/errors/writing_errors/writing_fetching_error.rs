use std::{env::VarError, fmt::{self, write}, io};

#[derive(Debug)]
pub enum WritingFetchingError {
    Db(postgres::Error),
    Id(uuid::Error),
}

impl From<postgres::Error> for WritingFetchingError {
    fn from(error: postgres::Error) -> Self {
        WritingFetchingError::Db(error)
    }
}

impl From<uuid::Error> for WritingFetchingError {
    fn from(value: uuid::Error) -> Self {
        WritingFetchingError::Id(value)
    }
}

impl fmt::Display for WritingFetchingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WritingFetchingError::Db(e) => write!(f, "Database Error: {}", e),
            WritingFetchingError::Id(msg) => write!(f, "Uuid Error: {}", msg),
        }
    }
}