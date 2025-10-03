pub fn parse_request_line(line: &str) -> Option<(&str, &str)> {
    
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 {
        Some((parts[0], parts[1]))
    } else {
        None
    }
}