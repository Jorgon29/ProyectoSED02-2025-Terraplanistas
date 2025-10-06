use std::{env::VarError, fmt::{self, write}, io};

#[derive(Debug)]
pub enum WritingCreationError {
    Db(postgres::Error),
    Validation(String),
    File(VarError),
    Io(io::Error)
}

impl From<postgres::Error> for WritingCreationError {
    fn from(error: postgres::Error) -> Self {
        WritingCreationError::Db(error)
    }
}

impl From<VarError> for WritingCreationError {
    fn from(value: VarError) -> Self {
        WritingCreationError::File(value)
    }
}

impl From<io::Error> for WritingCreationError {
    fn from(error: io::Error) -> Self {
        WritingCreationError::Io(error)
    }
}

impl fmt::Display for WritingCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WritingCreationError::Db(e) => write!(f, "Database Error: {}", e),
            WritingCreationError::Validation(msg) => write!(f, "Validation Error: {}", msg),
            WritingCreationError::File(e) => write!(f, "Filesystem error: {}", e),
            WritingCreationError::Io(e) => write!(f, "IO error: {}", e)
        }
    }
}