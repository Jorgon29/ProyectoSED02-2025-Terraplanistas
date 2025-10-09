pub enum ContentType {
    TextPlain,
    ApplicationJson,
    TextHtml,
    ImagePng,
    ImageJpeg,
    ImageSvg,
    ApplicationOctetStream,
    ApplicationJavascript,
    TextCss
}

impl ContentType {
    pub fn to_mime_type_str(&self) -> &'static str {
        match self {
            ContentType::TextPlain => "text/plain",
            ContentType::ApplicationJson => "application/json",
            ContentType::TextHtml => "text/html",
            ContentType::ImagePng => "image/png",
            ContentType::ImageJpeg => "image/jpeg",
            ContentType::ImageSvg => "image/svg+xml",
            ContentType::ApplicationOctetStream => "application/octet-stream",
            ContentType::ApplicationJavascript => "text/javascript",
            ContentType::TextCss => "text/css",
        }
    }
}

pub struct Answer {
    pub status_code: u16,
    pub response_body: Vec<u8>, 
    pub content_type: String, 
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

    pub fn new(status_code: u16, response_body: String, content_type: ContentType) -> Self {
        Answer {
            status_code,
            content_type: content_type.to_mime_type_str().to_string(), 
            response_body: response_body.into_bytes(), 
        }
    }
    
    pub fn new_binary(status_code: u16, response_body: Vec<u8>,content_type: ContentType) -> Self {
        Answer {
            status_code,
            content_type: content_type.to_mime_type_str().to_string(),
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