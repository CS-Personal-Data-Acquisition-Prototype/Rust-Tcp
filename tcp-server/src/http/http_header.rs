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

#[allow(unused)]
pub enum HttpHeaderType {
    SessionID,
    Cookie,
    SetCookie,
    DateTime,
    ContentType,
    ContentLength,
    // CORS Access Control (Ac) headers
    // Server
    AcAllowOrigin,
    AcAllowMethods,
    AcAllowHeaders,
    AcAllowCredentials,
    AcMaxAge,
    AcExposeHeaders,
    // Client
    Host,
    Origin,
    AcRequestMethod,
    AcRequestHeaders,
}

impl HttpHeaderType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            HttpHeaderType::SessionID => "session_id",
            HttpHeaderType::Cookie => "cookie",
            HttpHeaderType::SetCookie => "set-cookie",
            HttpHeaderType::DateTime => "datetime",
            HttpHeaderType::ContentType => "content-type",
            HttpHeaderType::ContentLength => "content-length",
            HttpHeaderType::AcAllowOrigin => "access-control-allow-origin",
            HttpHeaderType::AcAllowMethods => "access-control-allow-methods",
            HttpHeaderType::AcAllowHeaders => "access-control-allow-headers",
            HttpHeaderType::AcAllowCredentials => "access-control-allow-credentials",
            HttpHeaderType::AcMaxAge => "access-control-max-age",
            HttpHeaderType::AcExposeHeaders => "access-control-expose-headers",
            HttpHeaderType::Host => "host",
            HttpHeaderType::Origin => "origin",
            HttpHeaderType::AcRequestMethod => "access-control-request-method",
            HttpHeaderType::AcRequestHeaders => "access-control-request-headers",
        }
    }
}

pub struct HttpHeader {
    headers: HashMap<String, String>,
}

impl HttpHeader {
    pub const AC_ALLOWED_METHODS: &'static str = "GET, POST, PATCH, DELETE, OPTIONS";
    pub const AC_MAX_AGE: &'static str = "86400"; // Cache for 24 hours
    pub const AC_ORIGINS: [&str; 3] = ["http://localhost:8080", "http://localhost.:8080", "http://127.0.0.1:8080"];

    pub fn new() -> Self {
        HttpHeader {
            headers: HashMap::new(),
        }
    }

    pub fn build(self) -> Self {
        self
    }

    pub fn with(&mut self, other: Vec<(&str, &str)>) -> &mut Self {
        other.iter().for_each(|(key, value)| {
            self.insert(key.to_string(), value.to_string());
        });
        self
    }

    pub fn set_session(mut self, session_id: String) -> Self {
        self.insert(
            HttpHeaderType::SetCookie.as_str().to_string(),
            format!(
                "{}={}; HttpOnly; SameSite=Strict; Max-Age=3600; Path=/", //; Domain=<host> //TODO: add domain
                HttpHeaderType::SessionID.as_str(),
                session_id
            ),
        );
        self.insert(
            HttpHeaderType::AcExposeHeaders.as_str().to_string(),
            HttpHeaderType::SessionID.as_str().to_string(),
        );
        self
    }

    pub fn default(&mut self) -> &mut Self {
        self.with(vec![
            (HttpHeaderType::AcAllowCredentials.as_str(), "true"),
            (
                HttpHeaderType::DateTime.as_str(),
                //TODO: this isn't formatted correctly
                &Utc::now().timestamp().to_string(),
            ),
        ])
    }

    pub fn default_json() -> Self {
        let mut header = HttpHeader::new();
        header.default().with(vec![(
            HttpHeaderType::ContentType.as_str(),
            "application/json",
        )]);
        header.build()
    }

    pub fn default_html() -> Self {
        let mut header = HttpHeader::new();
        header.default().with(vec![(
            HttpHeaderType::ContentType.as_str(),
            "application/html",
        )]);
        header.build()
    }

    pub fn default_options() -> Self {
        let mut header = HttpHeader::new();
        header.default().with(vec![
            (
                HttpHeaderType::AcAllowMethods.as_str(),
                Self::AC_ALLOWED_METHODS,
            ),
            (
                HttpHeaderType::AcAllowHeaders.as_str(),
                &format!(
                    "{}, {}",
                    HttpHeaderType::ContentType.as_str(),
                    HttpHeaderType::SessionID.as_str()
                ),
            ),
            (HttpHeaderType::AcMaxAge.as_str(), Self::AC_MAX_AGE),
        ]);
        header.build()
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
