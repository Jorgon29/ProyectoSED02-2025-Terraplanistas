use std::fmt;

#[derive(Debug)]
pub enum UserCreationError {
    Db(postgres::Error),
    Validation(String),
}

impl From<postgres::Error> for UserCreationError {
    fn from(error: postgres::Error) -> Self {
        UserCreationError::Db(error)
    }
}

impl fmt::Display for UserCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserCreationError::Db(e) => write!(f, "Database Error: {}", e),
            UserCreationError::Validation(msg) => write!(f, "Validation Error: {}", msg),
        }
    }
}