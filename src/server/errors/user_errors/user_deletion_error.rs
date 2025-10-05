use std::fmt;

#[derive(Debug)]
pub enum UserDeletionError {
    Db(postgres::Error),
    Id(uuid::Error),
}

impl From<postgres::Error> for UserDeletionError {
    fn from(error: postgres::Error) -> Self {
        UserDeletionError::Db(error)
    }
}

impl fmt::Display for UserDeletionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserDeletionError::Db(e) => write!(f, "Database Error: {}", e),
            UserDeletionError::Id(msg) => write!(f, "Validation Error: {}", msg),
        }
    }
}