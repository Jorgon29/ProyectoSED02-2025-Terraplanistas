use dotenvy;
use postgres::{Client, Error, NoTls};

pub fn get_database_client()-> Result<Client, Error> {
    let host = dotenvy::var("DB_HOST").unwrap_or(String::from("localhost"));
    let db_name = dotenvy::var("DB_NAME").unwrap_or(String::from("SOME_DATABASE"));
    let db_user = dotenvy::var("DB_USER").unwrap_or(String::from("SOME_USER"));
    let db_pass = dotenvy::var("DB_PASS").unwrap_or(String::from("SOME_PASSWORD"));

    let connection_string = format!("host={} dbname={} user={} password={}", host, db_name, db_user, db_pass);
    Client::connect(connection_string.as_str(), NoTls)
}
