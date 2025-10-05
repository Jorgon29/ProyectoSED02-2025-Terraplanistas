use postgres::{Client, Error};
use uuid::Uuid;
use crate::server::errors::user_errors;

use crate::server::db::db_connect::get_database_client;
use crate::server::models::models_user::User;

pub fn add_user(name: &str, password: &str, email: &str) -> Result<u64, Error>{
    let mut client = get_database_client()?;
    Ok(client.execute("INSERT INTO schema_seguridad.APP_USER(name, password, email, role) VALUES ($1, $2, $3, (SELECT id FROM schema_seguridad.ROLE WHERE name = 'APP_USER'))", &[&name, &password, &email]))?
}

pub fn get_all_users() -> Result<Vec<User>, Error> {
    let mut client: Client = get_database_client()?;
    let mut result = Vec::new();
    let query = client.query("SELECT id, name FROM schema_seguridad.APP_USER", &[])?;
    for line in query {
        result.push(User{id: line.get(0), username: line.get(1)});
    };
    Ok(result)
}

pub fn delete_user(id: String) -> Result<u64, user_errors::user_deletion_error::UserDeletionError>{
    let id: Uuid = Uuid::parse_str(&id).map_err(user_errors::user_deletion_error::UserDeletionError::Id)?;
    let mut client: Client = get_database_client()?;
    let result = client.execute("DELETE FROM schema_seguridad.APP_USER WHERE id = $1", &[&id])?;
    Ok(result)
}