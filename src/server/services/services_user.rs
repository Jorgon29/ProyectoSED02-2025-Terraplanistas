use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use postgres::Error as DbError;

use crate::server::repositories::repositories_user;

pub fn create_user(username: &str, password: &str) -> Result<u64, DbError> {
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
    
    repositories_user::add_user(username, &password_hash_string)
}