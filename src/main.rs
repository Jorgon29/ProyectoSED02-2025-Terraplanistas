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

use crate::{server::controllers::answer::Answer, threadpool::ThreadPool};
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

    let mut answer = Answer::new(500, "".to_string());
    
    if let Some((method, path)) = routing_helpers::parse_request_line(&request_line) {
        let path_segments: Vec<&str> = path.trim_matches('/').split('/').collect();
        let resource = path_segments.get(0).unwrap_or(&"");

        answer = match *resource {
            "" | "index" => controllers::pages_controller::serve_html_file("src/client/views/index.html"),
            "assets" => controllers::pages_controller::serve_static_file(&path_segments),
            "roles" => controllers::controllers_role::handle_roles_request(method, &path_segments, &body_content_string),
            "users" => controllers::controllers_user::handle_users_request(method, &path_segments, &body_content_string),
            "writings" => {
                
                Answer::new(501, "Endpoint /writings no implementado aún".to_string())
            },
            _ => serve_html_file("client/views/index.html"),
        };
    }

    let response =
        format!("{}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", 
                answer.get_status_line(), 
                answer.get_body_size(), 
                answer.response_body);
    stream.write_all(response.as_bytes()).unwrap();
}

fn serve_html_file(file_path: &str) -> Answer {
    println!("📄 Sirviendo HTML: {}", file_path);
    
    match fs::read_to_string(file_path) {
        Ok(mut content) => {
            content = content.replace("{{mensaje}}", "Bienvenido desde Rust SSR!");
            
            Answer::new(200, content)
        }
        Err(e) => {
            println!("❌ Error al leer archivo HTML {}: {}", file_path, e);
            Answer::new(404, format!("gina no encontrada: {}", file_path))
        }
    }
}

fn serve_static_file(path_segments: &[&str]) -> Answer {
    if path_segments.len() < 2 {
        return Answer::new(404, "Archivo no especificado".to_string());
    }
    
    let full_path = path_segments[1..].join("/");
    let clean_path = full_path.split('?').next().unwrap_or(&full_path);
    let file_path = format!("src/client/assets/{}", clean_path);
    
    println!("📁 Sirviendo archivo estático: {}", file_path);
    
    match fs::read(&file_path) {
        Ok(content) => {
            let is_text_file = file_path.ends_with(".html") || 
                              file_path.ends_with(".css") || 
                              file_path.ends_with(".js") ||
                              file_path.ends_with(".jpeg") ||
                              file_path.ends_with(".json");
            
            if is_text_file {
                match String::from_utf8(content) {
                    Ok(text_content) => {
                        println!("✓ Archivo servido: {}", Path::new(&file_path).file_name().unwrap().to_string_lossy());
                        Answer::new(200, text_content)
                    }
                    Err(_) => Answer::new(500, "Error procesando archivo".to_string()),
                }
            } else {
                // Archivos binarios (imágenes, fuentes)
                let body = String::from_utf8_lossy(&content).to_string();
                println!("✓ Archivo binario servido: {}", Path::new(&file_path).file_name().unwrap().to_string_lossy());
                Answer::new(200, body)
            }
        }
        Err(e) => {
            println!("❌ Error al leer archivo estático {}: {}", file_path, e);
            Answer::new(404, format!("Archivo no encontrado: {}", clean_path))
        }
    }
}