use argon2::{
    password_hash::{rand_core::{Error, OsRng}, PasswordHasher, SaltString},
    Argon2,
};

use postgres;
use regex::Regex;

use crate::server::models::models_user::User;
use crate::server::repositories::repositories_user;
use crate::server::errors::user_errors::user_creation_error::UserCreationError;

pub fn create_user(username: &str, password: &str, email: &str) -> Result<u64, UserCreationError> {

    // RFC 5322 Official Standard - Obtenido de https://emailregex.com/
    const EMAIL_REGEX: &str = r##"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"##;
    const HAS_LOWERCASE: &str = r"[a-z]";
    const HAS_UPPERCASE: &str = r"[A-Z]";
    const HAS_DIGIT: &str = r"[0-9]";
    const HAS_SPECIAL: &str = r"[@!#$%&?]";
    const MIN_LENGTH: &str = r".{12,}";

    let re = match Regex::new(EMAIL_REGEX) {
        Ok(r) => r,
        Err(_) => {return Err(UserCreationError::Validation("Internal error: malformed email regex.".to_string()));}
    };

    if !re.is_match(email) {
        return Err(UserCreationError::Validation("Failed email format check".to_string()));
    }

    let re_length = match Regex::new(MIN_LENGTH) {
        Ok(r) => r,
        Err(_) => {
            return Err(UserCreationError::Validation(
                "Internal error: malformed password regex.".to_string()
            ));
        }
    };
    if !re_length.is_match(password) {
        return Err(UserCreationError::Validation(
            "Password must be at least 12 characters long.".to_string()
        ));
    }

    let re_lower = match Regex::new(HAS_LOWERCASE) {
        Ok(r) => r,
        Err(_) => {
            return Err(UserCreationError::Validation(
                "Internal error: malformed password regex.".to_string()
            ));
        }
    };
    if !re_lower.is_match(password) {
        return Err(UserCreationError::Validation(
            "Password must contain at least one lowercase letter.".to_string()
        ));
    }

    let re_upper = match Regex::new(HAS_UPPERCASE) {
        Ok(r) => r,
        Err(_) => {
            return Err(UserCreationError::Validation(
                "Internal error: malformed password regex.".to_string()
            ));
        }
    };
    if !re_upper.is_match(password) {
        return Err(UserCreationError::Validation(
            "Password must contain at least one uppercase letter.".to_string()
        ));
    }

    let re_digit = match Regex::new(HAS_DIGIT) {
        Ok(r) => r,
        Err(_) => {
            return Err(UserCreationError::Validation(
                "Internal error: malformed password regex.".to_string()
            ));
        }
    };
    if !re_digit.is_match(password) {
        return Err(UserCreationError::Validation(
            "Password must contain at least one digit.".to_string()
        ));
    }

    let re_special = match Regex::new(HAS_SPECIAL) {
        Ok(r) => r,
        Err(_) => {
            return Err(UserCreationError::Validation(
                "Internal error: malformed password regex.".to_string()
            ));
        }
    };
    if !re_special.is_match(password) {
        return Err(UserCreationError::Validation(
            "Password must contain at least one special character (@, !, #, $, %, &, ?).".to_string()
        ));
    }


    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

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

pub fn get_every_user() -> Result<Vec<User>, postgres::Error>{
    repositories_user::get_all_users()
}