use crate::server::controllers::answer::Answer;
use std::fs;
use std::path::Path;

pub fn serve_html_file(file_path: &str) -> Answer {
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

