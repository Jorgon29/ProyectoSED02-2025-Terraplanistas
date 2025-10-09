use crate::server::controllers::answer::{Answer, ContentType};
use std::fs;
use std::path::Path;



pub fn serve_static_file(path_segments: &[&str]) -> Answer {
    if path_segments.len() < 2 {
        return Answer::new(404, "File not specified".to_string(), ContentType::TextHtml);
    }
    
    let full_path = path_segments[2..].join("/");
    let clean_path = full_path.split('?').next().unwrap_or(&full_path);
    let file_path = format!("src/client/assets/{}", clean_path);
    
    match fs::read(&file_path) {
        Ok(content) => {
            let path = Path::new(clean_path);
            let content_type = get_content_type_from_path(&path);
            Answer::new_binary(200, content, content_type)
        }
        Err(e) => {
            Answer::new(404, format!("Archivo no encontrado: {}", e), ContentType::TextHtml)
        }
    }
}

pub fn get_content_type_from_path(path: &Path) -> ContentType {
    match path.extension().and_then(|ext| ext.to_str()) {
        
        Some("png") => ContentType::ImagePng,
        Some("jpg") | Some("jpeg") => ContentType::ImageJpeg,
        Some("svg") => ContentType::ImageSvg,
        
        Some("html") | Some("htm") => ContentType::TextHtml,
        Some("css") => ContentType::TextCss,
        Some("js") => ContentType::ApplicationJavascript,
        _ => ContentType::ApplicationOctetStream,
    }
}