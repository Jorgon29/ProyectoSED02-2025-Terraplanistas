use crate::server::db::db_connect::get_database_client;

pub fn get_roles() -> String {
    let mut client = get_database_client().unwrap();
        let mut result: String = String::new();
        for row in client.query("SELECT * FROM schema_seguridad.ROLE", &[]).expect("Failed to select role") {
            let name: &str = row.get(1);
            result.push_str(&format!("{} \n", name));
        }
        result
}