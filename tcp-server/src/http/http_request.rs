use serde_json::Value;
use std::str;
use url::form_urlencoded;

use super::{HttpHeader, HttpMethod, HttpPath};

#[allow(unused)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: HttpPath,
    pub parameters: Option<Vec<(String, String)>>,
    pub headers: HttpHeader,
    pub body: Option<Value>,
}

impl HttpRequest {
    #[allow(unused)]
    pub fn to_string(&self) -> String {
        format!(
            "{} {}{} HTTP/1.1\r\n{}\r\n\r\n{:#?}",
            self.method.to_string(),
            self.path.to_string(),
            self.parameters_to_string(),
            self.headers.to_string(),
            self.body,
        )
    }

    #[allow(unused)]
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

    #[allow(unused)]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    pub fn from_request_bytes(buffer: &[u8]) -> Self {
        //split the request on delimiter to separate the header and body, on err assume no body and set whole buffer to header
        let delimiter = b"\r\n\r\n";
        let (header, body_buffer) = buffer.split_at(
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

        str::from_utf8(header_bytes)
            .map_err(|error| format!("Error: {}", error))
            .unwrap_or("")
            .lines()
            .for_each(|line| {
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
                str::from_utf8(path_bytes)
                    .unwrap_or_default()
                    .trim()
                    .to_string(),
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

        let trim_body = String::from_utf8_lossy(&body_buffer[delimiter.len()..])
            .trim_end_matches('\0')
            .to_string();

        HttpRequest {
            method,
            path: HttpPath::from_string(path),
            parameters,
            headers,
            body: (trim_body.len() > 0).then(|| serde_json::from_str(&trim_body).unwrap()),
        }
    }

    #[allow(unused)]
    pub fn error() -> Self {
        HttpRequest {
            method: HttpMethod::Error,
            path: HttpPath::Index(String::new()),
            parameters: None,
            headers: HttpHeader::new(),
            body: None,
        }
    }

    #[allow(unused)]
    pub fn new(
        method: HttpMethod,
        path: HttpPath,
        parameters: Option<Vec<(String, String)>>,
        headers: HttpHeader,
        body: Option<Value>,
    ) -> Self {
        HttpRequest {
            method,
            path,
            parameters,
            headers,
            body,
        }
    }

    #[allow(unused)]
    pub fn get(
        path: HttpPath,
        parameters: Option<Vec<(String, String)>>,
        headers: HttpHeader,
        body: Option<Value>,
    ) -> Self {
        HttpRequest {
            method: HttpMethod::Get,
            path,
            parameters,
            headers,
            body,
        }
    }

    #[allow(unused)]
    pub fn post(
        path: HttpPath,
        parameters: Option<Vec<(String, String)>>,
        headers: HttpHeader,
        body: Option<Value>,
    ) -> Self {
        HttpRequest {
            method: HttpMethod::Post,
            path,
            parameters,
            headers,
            body,
        }
    }

    #[allow(unused)]
    pub fn patch(
        path: HttpPath,
        parameters: Option<Vec<(String, String)>>,
        headers: HttpHeader,
        body: Option<Value>,
    ) -> Self {
        HttpRequest {
            method: HttpMethod::Patch,
            path,
            parameters,
            headers,
            body,
        }
    }

    #[allow(unused)]
    pub fn delete(
        path: HttpPath,
        parameters: Option<Vec<(String, String)>>,
        headers: HttpHeader,
        body: Option<Value>,
    ) -> Self {
        HttpRequest {
            method: HttpMethod::Delete,
            path,
            parameters,
            headers,
            body,
        }
    }
}
