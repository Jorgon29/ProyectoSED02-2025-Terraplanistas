use crate::server::controllers::answer::{Answer, ContentType};
use crate::server::controllers::static_files_controller;
use std::fs;
use std::path::{Path, PathBuf};

pub fn pages_controller(method: &str, path_segments: &[&str], body: &str) -> Answer {
    if method == "GET" && path_segments.len() > 1 {
        let html_dir: &Path = Path::new("src/client/views");
        match path_segments[1] {
            "home" => {
                let home_dir = html_dir.join(Path::new("index.html"));
                let content = fs::read(home_dir);
                return match content {
                    Ok(f) => Answer::new_binary(200, f, ContentType::TextHtml),
                    Err(_) => Answer::new(500, "Failed to read home page".to_string(), ContentType::TextHtml)
                }
            },
            "assets" => {
                static_files_controller::serve_static_file(path_segments)
            },
            _ => return Answer::new(404, "Not found".to_string(), ContentType::TextHtml)
        }
    } else {
        Answer::new(403, "Forbidden method for pages".to_string(), ContentType::TextHtml)
    }
}

pub fn serve_html_file(file_path: &str) -> Answer {
    println!("📄 Sirviendo HTML: {}", file_path);
    
    match fs::read_to_string(file_path) {
        Ok(mut content) => {
            content = content.replace("{{mensaje}}", "Bienvenido desde Rust SSR!");
            
            Answer::new(200, content, ContentType::TextHtml)
        }
        Err(e) => {
            println!("❌ Error al leer archivo HTML {}: {}", file_path, e);
            Answer::new(404, format!("gina no encontrada: {}", file_path), ContentType::TextHtml)
        }
    }
}
