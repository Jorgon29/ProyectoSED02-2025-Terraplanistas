use crate::server::controllers::answer::Answer;
use std::fs;
use std::path::Path;

pub fn serve_static_file(path_segments: &[&str]) -> Answer {
    if path_segments.len() < 2 {
        return Answer::new(404, "Archivo no especificado".to_string());
    }
    
    // Reconstruir la ruta del archivo (eliminar parámetros query)
    let full_path = path_segments[1..].join("/");
    let clean_path = full_path.split('?').next().unwrap_or(&full_path);
    let file_path = format!("client/assets/{}", clean_path);
    
    println!("📁 StaticFilesController - Sirviendo archivo: {}", file_path);
    
    match fs::read(&file_path) {
        Ok(content) => {
            let is_text_file = file_path.ends_with(".html") || 
                              file_path.ends_with(".css") || 
                              file_path.ends_with(".js") ||
                              file_path.ends_with(".json");
            
            if is_text_file {
                match String::from_utf8(content) {
                    Ok(text_content) => {
                        println!("✓ StaticFilesController - Archivo servido: {}", Path::new(&file_path).file_name().unwrap().to_string_lossy());
                        Answer::new(200, text_content)
                    }
                    Err(_) => Answer::new(500, "Error procesando archivo".to_string()),
                }
            } else {
                // Archivos binarios
                let body = String::from_utf8_lossy(&content).to_string();
                println!("✓ StaticFilesController - Archivo binario servido: {}", Path::new(&file_path).file_name().unwrap().to_string_lossy());
                Answer::new(200, body)
            }
        }
        Err(e) => {
            println!("❌ StaticFilesController - Error al leer archivo {}: {}", file_path, e);
            Answer::new(404, format!("Archivo no encontrado: {}", clean_path))
        }
    }
}