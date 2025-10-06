pub fn parse_request_line(line: &str) -> Option<(&str, &str)> {
    
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 {
        Some((parts[0], parts[1]))
    } else {
        None
    }
}

fn simulate_multipart_parse(raw_bytes: Vec<u8>) -> Result<(String, Vec<u8>, String), String> {
    let image_data = &raw_bytes;
    Ok(("".to_string(), raw_bytes, "png".to_string()))
}
