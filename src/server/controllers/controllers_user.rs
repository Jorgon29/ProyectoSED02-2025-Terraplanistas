use crate::server::services::services_user;

pub fn handle_users_request(method: &str, path_segments: &[&str], body: &str) -> (String, String) {
    match method {
        "GET" => {
            if path_segments.len() == 1 {
                // Todos los usuarios
                ("HTTP/1.1 200 OK".to_owned(), "Fetched all users (Controller)".to_owned())
            } else if path_segments.len() == 2 {
                // Uno específico
                let id = path_segments[1];
                ("HTTP/1.1 200 OK".to_owned(), format!("Fetched user with ID: {}", id))
            } else {
                ("HTTP/1.1 400 Bad Request".to_owned(), "Invalid users URL format.".to_owned())
            }
        }
        "PUT" => {
            if path_segments.len() == 2 {
                // actualizar?
                let id = path_segments[1];
                ("HTTP/1.1 200 OK".to_owned(), format!("Updated user with ID: {}", id))
            } else {
                ("HTTP/1.1 400 Bad Request".to_owned(), "Update requires a user ID.".to_owned())
            }
        }
        "POST" => {
            if path_segments.len() == 1 {
                let data = body.split(",");
                let parts_data: Vec<&str> = data.collect();
                if parts_data.len() == 3 {
                    let username = parts_data[0];
                    let password = parts_data[1];
                    let email = parts_data[2];
                    let insert_query = services_user::create_user(username, password, email);
                    let answer: (String, String) = match insert_query {
                        Ok(_) => ("HTTP/1.1 201 OK".to_owned(), "User created".to_owned()),
                        _error => ("HTTP/1.1 500 Internal server error".to_owned(), "Failed to connect to database on user creation".to_owned())
                    };
                    return answer;
                }
            }
            ("HTTP/1.1 400 Bad Request".to_owned(), "Bad request for creating user".to_owned())
        }
        _ => {
            ("HTTP/1.1 405 Method Not Allowed".to_owned(), "Method not supported for /users.".to_owned())
        }
    }
}