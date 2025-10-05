use crate::server::{controllers::answer::Answer, services::services_user};

pub fn handle_users_request(method: &str, path_segments: &[&str], body: &str) -> Answer {
    match method {
        "GET" => {
            if path_segments.len() == 1 {
                let selection = services_user::get_every_user();
                let answer = match selection {
                    Ok(users) => {
                        let mut data = String::new();
                        for user in users {
                            data.push_str(&user.id.to_string());
                            data.push(',');
                            data.push_str(&user.username);
                            data.push_str("\n");
                        }
                        Answer::new(200, data)
                    },
                    Err(e) => Answer::new(500, format!("Failed while attemting to create user: {}", e))
                };
                return answer;
            } else if path_segments.len() == 2 {
                // Uno específico
                let id = path_segments[1];
                Answer::new(200, format!("Fetched user with ID: {}", id))
            } else {
                Answer::new(400, "Invalid users URL format.".to_owned())
            }
        }
        "PUT" => {
            if path_segments.len() == 2 {
                // actualizar?
                let id = path_segments[1];
                Answer::new(200, format!("Updated user with ID: {}", id))
            } else {
                Answer::new(400, "Update requires a user ID.".to_owned())
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
                    let answer: Answer = match insert_query {
                        Ok(_) => Answer::new(201, "User created".to_owned()),
                        Err(e) => Answer::new(500, format!("Failed while attemting to create user: {}", e))
                    };
                    return answer;
                }
            }
            Answer::new(400, "Bad request for creating user".to_owned())
        }
        _ => {
            Answer::new(405, "Method not supported for /users.".to_owned())
        }
    }
}