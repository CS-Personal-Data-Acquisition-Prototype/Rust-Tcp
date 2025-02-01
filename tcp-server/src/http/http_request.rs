use std::str;
use url::form_urlencoded;

use super::{HttpHeader, HttpMethod};

pub struct HttpRequest {//TODO: make path, paramaters, and maybe body Types to allow pattern matching
    pub method: HttpMethod,
    pub path: String,
    pub parameters: Option<Vec<(String, String)>>,
    pub headers: HttpHeader,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn to_string(&self) -> String {
        format!(
            "{} {}{} HTTP/1.1\r\n{}\r\n\r\n{}",
            self.method.to_string(),
            self.path,
            self.parameters_to_string(),
            self.headers.to_string(),
            self.body.as_ref().unwrap_or(&String::from("Error converting request body to String.")),
        )
    }

    //Returns a String in the format of "?key1=val1&keyN=valN" or "" if parameters is empty
    pub fn parameters_to_string(&self) -> String {
        self.parameters
            .as_ref()
            .filter(|params| !params.is_empty())
            .map(|params| {
                format!(
                    "?{}",
                    params
                        .iter()
                        .map(|(key, val)| format!("{key}={val}"))
                        .collect::<Vec<String>>()
                        .join("&")
                )
            })
            .unwrap_or_default()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    pub fn from_request_bytes(buffer: &[u8]) -> Self {
        //split the request on delimiter to separate the header and body, on err assume no body and set whole buffer to header
        let delimiter = b"\r\n\r\n";
        let (header, body) = buffer.split_at(
            buffer
                .windows(delimiter.len())
                .position(|window| window == delimiter)
                .unwrap_or_else(|| buffer.len()),
        );

        //split the header on spaces ' '
        let (method, whole_path, header_bytes) = {
            let mut split = header.splitn(3, |&byte| byte == b' ');
            (
                HttpMethod::from_bytes(split.next().unwrap_or_default()),
                split.next().unwrap_or_default(),
                &split.next().unwrap_or_default()[10..],
            )
        };

        let mut headers = HttpHeader::new();
        
        str::from_utf8(header_bytes).map_err(|error| {
            format!("Error: {}", error)
        }).unwrap_or("").lines().for_each(|line| {
            let (k, v) = line.split_once(':').unwrap_or(("", ""));
            headers.insert(String::from(k), String::from(v));
        });

        let (path, parameters): (String, Option<Vec<(String, String)>>) = {
            //split array at pos of '?' or end of byte array
            let (path_bytes, query_string_bytes) = whole_path.split_at(
                whole_path
                    .iter()
                    .position(|&b| b == b'?')
                    .unwrap_or(whole_path.len()),
            );

            (
                str::from_utf8(path_bytes).unwrap_or_default().to_string(),
                Some(
                    form_urlencoded::parse(if query_string_bytes.is_empty() {
                        query_string_bytes
                    } else {
                        &query_string_bytes[1..]
                    })
                    .into_owned()
                    .collect(),
                ),
            )
        };

        HttpRequest {
            method,
            path,
            parameters,
            headers,
            body: (body.len() > delimiter.len())
                .then(|| String::from_utf8_lossy(&body[delimiter.len()..]).into_owned()),
        }
    }

    pub fn error() -> Self {
        HttpRequest {
            method: HttpMethod::Error,
            path: String::new(),
            parameters: None,
            headers: HttpHeader::new(),
            body: None,
        }
    }

    pub fn new(
        method: HttpMethod,
        path: String,
        parameters: Option<Vec<(String, String)>>,
        headers: HttpHeader,
        body: Option<String>,
    ) -> Self {
        HttpRequest {
            method,
            path,
            parameters,
            headers,
            body,
        }
    }

    pub fn default_get() -> Self {
        HttpRequest::get(String::from("/"), None, HttpHeader::default_json(String::new()), None)
    }

    pub fn default_post() -> Self {
        HttpRequest::post(String::from("/"), None, HttpHeader::default_json(String::new()), None)
    }

    pub fn default_patch() -> Self {
        HttpRequest::patch(String::from("/"), None, HttpHeader::default_json(String::new()), None)
    }

    pub fn default_delete() -> Self {
        HttpRequest::delete(String::from("/"), None, HttpHeader::default_json(String::new()), None)
    }

    pub fn get(
        path: String,
        parameters: Option<Vec<(String, String)>>,
        headers: HttpHeader,
        body: Option<String>,
    ) -> Self {
        HttpRequest {
            method: HttpMethod::Get,
            path,
            parameters,
            headers,
            body,
        }
    }

    pub fn post(
        path: String,
        parameters: Option<Vec<(String, String)>>,
        headers: HttpHeader,
        body: Option<String>,
    ) -> Self {
        HttpRequest {
            method: HttpMethod::Post,
            path,
            parameters,
            headers,
            body,
        }
    }

    pub fn patch(
        path: String,
        parameters: Option<Vec<(String, String)>>,
        headers: HttpHeader,
        body: Option<String>,
    ) -> Self {
        HttpRequest {
            method: HttpMethod::Patch,
            path,
            parameters,
            headers,
            body,
        }
    }

    pub fn delete(
        path: String,
        parameters: Option<Vec<(String, String)>>,
        headers: HttpHeader,
        body: Option<String>,
    ) -> Self {
        HttpRequest {
            method: HttpMethod::Delete,
            path,
            parameters,
            headers,
            body,
        }
    }

    //TODO testing function
    pub fn test() {
        HttpRequest::new(
            HttpMethod::Get,
            String::from("/"),
            Some(vec![]),
            HttpHeader::default_json(String::new()),
            None
        );
    }
}
