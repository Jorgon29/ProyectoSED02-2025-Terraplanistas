pub mod server;
pub mod client;

use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    env,
    path::Path
};

pub mod routing_helpers;
pub mod threadpool;

use crate::{server::controllers::answer::{Answer, ContentType}, threadpool::ThreadPool};
use crate::server::controllers;

fn main() {
    let api_address = env::var("ADDRESS").unwrap_or(String::from("127.0.0.1:7878"));
    let listener = TcpListener::bind(api_address).unwrap();
    let pool = ThreadPool::new(4);

    println!("Servidor corriendo en http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    
    let mut headers = Vec::new();
    let mut content_type: Option<String> = None;
    let mut content_length: usize = 0;
    let mut request_line_option: Option<String> = None;

    for line_result in (&mut buf_reader).lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(_) => return,
        };

        if line.is_empty() {
            break;
        }

        if request_line_option.is_none() {
            request_line_option = Some(line.clone());
        }

        let line_lower = line.to_lowercase();

        if line_lower.starts_with("content-length:") {
            if let Some((_, len_str)) = line.split_once(':') {
                if let Ok(length) = len_str.trim().parse::<usize>() {
                    content_length = length;
                }
            }
        } else if line_lower.starts_with("content-type:") {
            if let Some((_, type_str)) = line.split_once(':') {
                content_type = Some(type_str.trim().to_owned());
            }
        }
        headers.push(line);
    }
    
    let request_line = match request_line_option {
        Some(line) => line,
        None => return,
    };
    
    let mut body_content_string = String::new();
    let mut body_content_bytes: Vec<u8> = Vec::new();

    if content_length > 0 {
        let mut buffer = vec![0; content_length];
        if buf_reader.read_exact(&mut buffer).is_ok() {
            let is_binary_payload = content_type.as_deref().map(|s| {
                s.starts_with("multipart/form-data") || s.starts_with("application/octet-stream")
            }).unwrap_or(false);
        
            if is_binary_payload {
                body_content_bytes = buffer;
            } else if let Ok(s) = String::from_utf8(buffer) {
                body_content_string = s;
            } else {
                return; 
            }
        } else {
            return;
        }
    }

    println!("Solicitud recibida: {}", request_line);

    let mut answer = Answer::new(500, "".to_string(), ContentType::TextHtml);
    
    if let Some((method, path)) = routing_helpers::parse_request_line(&request_line) {
        let path_segments: Vec<&str> = path.trim_matches('/').split('/').collect();
        let resource = path_segments.get(0).unwrap_or(&"");

        answer = match *resource {
            "assets" => controllers::static_files_controller::serve_static_file(&path_segments),
            "roles" => controllers::controllers_role::handle_roles_request(method, &path_segments, &body_content_string),
            "users" => controllers::controllers_user::handle_users_request(method, &path_segments, &body_content_string),
            "writings" => {
                Answer::new(501, "Endpoint /writings no implementado aún".to_string(), ContentType::TextHtml)
            },
            "pages" => controllers::controllers_page::pages_controller(method, &path_segments, &body_content_string),
            _ => Answer::new(403, "Forbidden route or not available".to_string(), ContentType::TextHtml)
        };
    }

    let headers = format!(
    "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", 
    answer.get_status_line(), 
    answer.content_type,
    answer.get_body_size()
    );

    stream.write_all(headers.as_bytes()).unwrap();

    stream.write_all(&answer.response_body).unwrap();
}