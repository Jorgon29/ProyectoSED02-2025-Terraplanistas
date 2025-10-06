use uuid::Uuid;


use crate::server::{controllers::answer::{self, Answer}, services::services_writing::save_writing};

const FILE_SEPARATOR: &[u8] = b"---FILE_SEPARATOR---";

pub fn handle_writing_request(
    method: &str, 
    path_segments: &[&str], 
    body_content_string: &str,
    image_data_raw_body: Option<Vec<u8>>,
    image_extension: &str,
) -> Answer {
    
    if method != "POST" {
        return Answer::new(405, "Method not allowed for /writings.".to_string());
    }
    let raw_body = match image_data_raw_body {
        Some(data) => data,
        None => return Answer::new(400, "Image file not received.".to_string()),
    };
    
    let separator_position = raw_body.windows(FILE_SEPARATOR.len())
        .position(|window| window == FILE_SEPARATOR)
        .map(|pos| pos + FILE_SEPARATOR.len());

    let (metadata_bytes, cover_data) = match separator_position {
        Some(sep_end) => {
            let metadata = &raw_body[..sep_end - FILE_SEPARATOR.len()];
            let image = &raw_body[sep_end..];
            (metadata, image)
        },
        None => {
            return Answer::new(400, "Missing required file separator in body.".to_string());
        }
    };

    let body_metadata = match String::from_utf8(metadata_bytes.to_vec()) {
        Ok(s) => s,
        Err(_) => return Answer::new(400, "Metadata is not valid UTF-8.".to_string()),
    };
    
    let parts_data: Vec<&str> = body_metadata.split("###").collect();
    if parts_data.len() != 5 {
        return Answer::new(400, "Improper separation of data: '###' separated values.".to_string());
    }

    let author = match Uuid::parse_str(parts_data[0]) {Ok(u) => u, Err(_) => return Answer::new(400, "Invalid author UUID.".to_string()), };
    let w_type = match Uuid::parse_str(parts_data[1]) {Ok(u) => u, Err(_) => return Answer::new(400, "Invalid writing type UUID.".to_string()), };
    let title = parts_data[2].to_string();
    let content = parts_data[3].to_string();
    let tags_csv = parts_data[4];

    let mut memory_tags: Vec<Uuid> = Vec::new();

    for tag in tags_csv.split(',') {
        if tag.trim().is_empty() { continue; }
        match Uuid::parse_str(tag.trim()) {
            Ok(id) => memory_tags.push(id),
            Err(_) => return Answer::new(400, format!("Invalid tag UUID: {}", tag)),
        }
    }

    let result = save_writing(
        author, 
        title, 
        content, 
        memory_tags, 
        cover_data.to_vec(),
        w_type, 
        image_extension.to_string()
    );

    match result {
        Ok(_) => Answer::new(201, "Created story".to_string()),
        Err(e) => Answer::new(500, format!("Failed to create writing: {}", e)),
    }
}