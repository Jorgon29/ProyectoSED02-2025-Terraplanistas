pub struct Answer {
    pub status_code: u16,
    pub response_body: String,
}

impl Answer {
    fn get_status_phrase(code: u16) -> &'static str {
        match code {
            200 => "OK",
            201 => "Created",
            400 => "Bad Request",
            404 => "Not Found",
            405 => "Method Not Allowed",
            500 => "Internal Server Error",
            _ => "Unknown Status"
        }
    }

    pub fn new(status_code: u16, response_body: String) -> Self {
        Answer {
            status_code,
            response_body,
        }
    }

    pub fn get_status_line(&self) -> String {
        let phrase = Self::get_status_phrase(self.status_code);
        format!("HTTP/1.1 {} {}", self.status_code, phrase)
    }

    pub fn get_body_size(&self) -> usize {
        self.response_body.len()
    }
}