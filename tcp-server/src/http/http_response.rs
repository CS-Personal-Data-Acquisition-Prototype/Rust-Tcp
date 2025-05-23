use std::{io::Write, net::TcpStream, sync::Mutex};

use serde_json::json;

use super::{HttpHeader, HttpHeaderType, HttpStatus};

pub struct HttpResponse {
    pub status: HttpStatus,
    pub headers: Mutex<HttpHeader>,
    pub body: String,
}

impl HttpResponse {
    pub fn new(status: HttpStatus, header: HttpHeader, body: String) -> HttpResponse {
        HttpResponse {
            status,
            headers: Mutex::new(header),
            body: body.trim_end_matches('\0').to_string(),
        }
    }

    pub fn from_vec(body: String) -> HttpResponse {
        HttpResponse::new(HttpStatus::OK, HttpHeader::default_json(), body)
    }

    pub fn to_string(&self) -> String {
        format!(
            "HTTP/1.1 {}\r\n{}: {}\r\n{}\r\n\r\n{}",
            self.status.as_str(),
            HttpHeaderType::ContentLength.as_str(),
            self.body.len(),
            self.headers.lock().unwrap().to_string(),
            self.body,
        )
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    pub fn send(&self, mut stream: TcpStream) -> Result<(), String> {
        let data: &[u8] = &self.to_bytes();
        stream
            .write_all(data)
            .map_err(|e| format!("Failed to send data (attempted {} bytes): {e}", data.len()))?;
        stream
            .flush()
            .map_err(|e| format!("Failed to flush after sending data: {e}"))?;
        Ok(())
    }

    pub fn html_404() -> HttpResponse {
        HttpResponse::new(
            HttpStatus::NotFound,
            HttpHeader::default_html(),
            String::from("<html><body><h1>404 Not Found</h1></body></html>"),
        )
    }

    pub fn json_404(resource: &str) -> HttpResponse {
        HttpResponse::new(
            HttpStatus::NotFound,
            HttpHeader::default_json(),
            json!({"error": format!("{resource} not found")}).to_string(),
        )
    }

    pub fn options_response() -> HttpResponse {
        HttpResponse::new(
            HttpStatus::NoContent,
            HttpHeader::default_options(),
            String::new(),
        )
    }

    pub fn bad_request(error_msg: &str) -> HttpResponse {
        HttpResponse::new(
            HttpStatus::BadRequest,
            HttpHeader::default_json(),
            json!({"error": error_msg}).to_string(),
        )
    }

    pub fn missing_body(msg: Option<&str>) -> HttpResponse {
        HttpResponse::bad_request(
            format!("Missing request body.{}", msg.unwrap_or_default()).as_str(),
        )
    }

    pub fn invalid_body(msg: Option<&str>) -> HttpResponse {
        HttpResponse::bad_request(
            format!("Invalid request body.{}", msg.unwrap_or_default()).as_str(),
        )
    }

    pub fn not_authorized() -> HttpResponse {
        HttpResponse::new(
            HttpStatus::Unauthorized,
            HttpHeader::default_json(),
            json!({"error": "Invalid authentication credentials."}).to_string(),
        )
    }

    pub fn forbidden() -> HttpResponse {
        HttpResponse::new(
            HttpStatus::Forbidden,
            HttpHeader::default_json(),
            json!({"error": "User not authorized."}).to_string(),
        )
    }

    pub fn no_content() -> HttpResponse {
        HttpResponse::new(
            HttpStatus::NoContent,
            HttpHeader::default_json(),
            String::new(),
        )
    }
}
