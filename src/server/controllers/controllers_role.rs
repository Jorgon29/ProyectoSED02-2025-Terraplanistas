pub fn handle_roles_request(method: &str, path_segments: &[&str], body: &str) -> (String, String) {
    match method {
        "GET" => {
            // Path segments: ["roles", "...", "..."]
            if path_segments.len() == 1 {
                // Todos los roles
                ("HTTP/1.1 200 OK".to_owned(), "Fetched all roles (Controller)".to_owned())
            } else if path_segments.len() == 2 {
                // Uno específico
                let id = path_segments[1];
                ("HTTP/1.1 200 OK".to_owned(), format!("Fetched role with ID: {}", id))
            } else {
                ("HTTP/1.1 400 Bad Request".to_owned(), "Invalid roles URL format.".to_owned())
            }
        }
        "POST" => {
            ("HTTP/1.1 201 Created".to_owned(), format!("Attempting to create role with body: {}", body))
        }
        _ => {
            ("HTTP/1.1 405 Method Not Allowed".to_owned(), "Method not supported for /roles.".to_owned())
        }
    }
}
