use std::env;
use postgres::{Client, Error, NoTls};

pub fn get_database_client()-> Result<Client, Error> {
    let host = env::var("DB_HOST").unwrap_or(String::from("localhost"));
    let db_name = env::var("DB_NAME").unwrap_or(String::from("SOME_DATABASE"));
    let db_user = env::var("DB_USER").unwrap_or(String::from("SOME_USER"));
    let db_pass = env::var("DB_PASS").unwrap_or(String::from("SOME_PASSWORD"));

    let connection_string = format!("host={} dbname={} user={} password={}", host, db_name, db_user, db_pass);
    Client::connect(connection_string.as_str(), NoTls)
}
