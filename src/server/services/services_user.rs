use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use regex::Regex;

use crate::server::repositories::repositories_user;
use crate::server::errors::user_errors::user_creation_error::UserCreationError;

pub fn create_user(username: &str, password: &str, email: &str) -> Result<u64, UserCreationError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    const EMAIL_REGEX: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";

    let re = match Regex::new(EMAIL_REGEX) {
        Ok(r) => r,
        Err(_) => {return Err(UserCreationError::Validation("Internal error: malformed email regex.".to_string()));}
    };

    if !re.is_match(email) {
        return Err(UserCreationError::Validation("Failed email format check".to_string()));
    }

    let hash_result = argon2.hash_password(password.as_bytes(), &salt);

    let password_hash_string = match hash_result {
        Ok(hash) => hash.to_string(),
        Err(e) => {
            eprintln!("Password hashing failed: {}", e);
            panic!("Critical Hashing Failure: {}", e); 
        }
    };
    
    repositories_user::add_user(username, &password_hash_string, email).map_err(UserCreationError::Db)
}