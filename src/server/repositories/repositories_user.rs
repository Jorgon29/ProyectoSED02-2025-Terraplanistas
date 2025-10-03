use postgres::Error;

use crate::server::db::db_connect::get_database_client;

pub fn add_user(name: &str, password: &str) -> Result<u64, Error>{
    let mut client = get_database_client().unwrap();
    Ok(client.execute("INSERT INTO schema_seguridad.APP_USER(name, password, role) VALUES ($1, $2, (SELECT id FROM schema_seguridad.ROLE WHERE name = 'APP_USER'))", &[&name, &password]))?
}