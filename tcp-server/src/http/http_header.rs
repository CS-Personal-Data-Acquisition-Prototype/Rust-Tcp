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
    pub const fn as_str(&self) -> &'static str {
        match self {
            HttpStatus::OK => "200 OK",
            HttpStatus::Created => "201 Created",
            HttpStatus::NoContent => "204 No Content",
            HttpStatus::BadRequest => "400 Bad Request",
            HttpStatus::Unauthorized => "401 Unauthorized",
            HttpStatus::Forbidden => "403 Forbidden",
            HttpStatus::NotFound => "404 Not Found",
        }
    }
}

pub enum HttpHeaderType {
    SessionID,
    Cookie,
    ContentLength,
}

impl HttpHeaderType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            HttpHeaderType::SessionID => "session_id",
            HttpHeaderType::Cookie => "Cookie",
            HttpHeaderType::ContentLength => "Content-Length",
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

    #[allow(unused)]
    pub fn with(mut head: Self, other: Vec<(String, String)>) -> Self {
        other.iter().for_each(|(key, value)| {
            head.insert(key.clone(), value.clone());
        });
        head
    }

    pub fn set_session(mut self, session_id: String) -> Self {
        self.insert(
            String::from("Set-Cookie"),
            format!(
                "{}={}; HttpOnly; SameSite=Strict; Max-Age=3600; Path=/", //; Domain=<host>
                HttpHeaderType::SessionID.as_str(),
                session_id
            ),
        );
        self
    }

    pub fn default(content_type: String) -> Self {
        let mut header = HttpHeader::new();

        header.insert(String::from("Content-Type"), content_type);
        header.insert(String::from("Datetime"), Utc::now().timestamp().to_string()); //TODO: this isn't working correctly

        header
    }

    pub fn default_json() -> Self {
        HttpHeader::default(String::from("application/json"))
    }

    pub fn default_html() -> Self {
        HttpHeader::default(String::from("application/html"))
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

    pub fn get(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }

    pub fn get_cookie(&self, key: &str) -> Option<String> {
        match self.get(HttpHeaderType::Cookie.as_str()) {
            Some(cookie_str) => match cookie_str.split_once(key) {
                Some((_, split_cookie)) => match split_cookie[1..].split_once(';') {
                    Some((cookie_value, _)) => Some(cookie_value.to_string()),
                    None => Some(split_cookie[1..].to_string()),
                },
                None => None,
            },
            None => None,
        }
    }
}
