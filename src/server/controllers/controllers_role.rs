use crate::server::controllers::answer::{self, Answer, ContentType};

pub fn handle_roles_request(method: &str, path_segments: &[&str], body: &str) -> Answer {
    match method {
        "GET" => {
            // Path segments: ["roles", "...", "..."]
            if path_segments.len() == 1 {
                // Todos los roles
                Answer::new(200, "Fetched all roles (Controller)".to_owned(), ContentType::TextHtml)
            } else if path_segments.len() == 2 {
                // Uno específico
                let id = path_segments[1];
                Answer::new(200, format!("Fetched role with ID: {}", id), ContentType::TextHtml)
            } else {
                Answer::new(400, "Invalid roles URL format.".to_owned(), ContentType::TextHtml)
            }
        }
        "POST" => {
            Answer::new(201, format!("Attempting to create role with body: {}", body), ContentType::TextHtml)
        }
        _ => {
            Answer::new(405, "Method not supported for /roles.".to_owned(), ContentType::TextHtml)
        }
    }
}
