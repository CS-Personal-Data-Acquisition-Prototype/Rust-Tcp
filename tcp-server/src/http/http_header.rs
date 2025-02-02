use chrono::Utc;
use std::collections::HashMap;

//statuses the server uses
pub enum HttpStatus {
    OK = 200,
    Created = 201,
    NoContent = 204,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
}

impl HttpStatus {
    pub fn to_string(&self) -> String {
        match self {
            HttpStatus::OK => String::from("200 OK"),
            HttpStatus::Created => String::from("201 Created"),
            HttpStatus::NoContent => String::from("204 No Content"),
            HttpStatus::BadRequest => String::from("400 Bad Request"),
            HttpStatus::Unauthorized => String::from("401 Unauthorized"),
            HttpStatus::Forbidden => String::from("403 Forbidden"),
            HttpStatus::NotFound => String::from("404 Not Found"),
        }
    }
}

pub struct HttpHeader {
    headers: HashMap<String, String>,
}

impl HttpHeader {
    pub fn new() -> Self {
        HttpHeader {
            headers: HashMap::new(),
        }
    }

    pub fn default(content_type: String, session_id: String) -> Self {
        let mut header = HttpHeader::new();

        header
            .headers
            .insert(String::from("Content-Type"), content_type);
        header
            .headers
            .insert(String::from("session_id"), session_id);
        header
            .headers
            .insert(String::from("Datetime"), Utc::now().timestamp().to_string()); //TODO: this isn't working correctly

        header
    }

    pub fn default_json(session_id: String) -> Self {
        HttpHeader::default(String::from("application/json"), session_id)
    }

    pub fn default_html(session_id: String) -> Self {
        HttpHeader::default(String::from("application/html"), session_id)
    }

    pub fn to_string(&self) -> String {
        self.headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<String>>()
            .join("\r\n")
    }

    pub fn insert(&mut self, key: String, value: String) -> Option<String> {
        self.headers.insert(key, value)
    }

    pub fn get(&self, key: String) -> Option<&String> {
        self.headers.get(&key)
    }
}
