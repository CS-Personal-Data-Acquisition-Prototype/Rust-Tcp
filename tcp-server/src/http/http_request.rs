use std::str;
use url::form_urlencoded;

use super::HttpMethod;

pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub parameters: Option<Vec<(String, String)>>,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn to_string(&self) -> String {
        format!(
            "{} {}{} HTTP/1.1\r\nContent Length: {}\r\n\r\n{}",
            self.method.to_string(),
            self.path,
            self.parameters_to_string(),
            self.body.as_ref().unwrap_or(&String::new()).len(),
            self.body.as_ref().unwrap_or(&String::new()),
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
        let (method, whole_path) = {
            let mut split = header.splitn(3, |&byte| byte == b' ');
            (
                HttpMethod::from_bytes(split.next().unwrap_or_default()),
                split.next().unwrap_or_default(),
            )
        };

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
            body: (body.len() > delimiter.len())
                .then(|| String::from_utf8_lossy(&body[delimiter.len()..]).into_owned()),
        }
    }

    pub fn error() -> Self {
        HttpRequest {
            method: HttpMethod::Error,
            path: String::new(),
            body: None,
            parameters: None,
        }
    }

    pub fn new(
        method: HttpMethod,
        path: String,
        parameters: Option<Vec<(String, String)>>,
        body: Option<String>,
    ) -> Self {
        HttpRequest {
            method,
            path,
            parameters,
            body,
        }
    }

    pub fn default_get() -> Self {
        HttpRequest::get("/".to_string(), None, None)
    }

    pub fn default_post() -> Self {
        HttpRequest::post("/".to_string(), None, None)
    }

    pub fn default_patch() -> Self {
        HttpRequest::patch("/".to_string(), None, None)
    }

    pub fn default_delete() -> Self {
        HttpRequest::delete("/".to_string(), None, None)
    }

    pub fn get(
        path: String,
        parameters: Option<Vec<(String, String)>>,
        body: Option<String>,
    ) -> Self {
        HttpRequest {
            method: HttpMethod::Get,
            path,
            parameters,
            body,
        }
    }

    pub fn post(
        path: String,
        parameters: Option<Vec<(String, String)>>,
        body: Option<String>,
    ) -> Self {
        HttpRequest {
            method: HttpMethod::Post,
            path,
            parameters,
            body,
        }
    }

    pub fn patch(
        path: String,
        parameters: Option<Vec<(String, String)>>,
        body: Option<String>,
    ) -> Self {
        HttpRequest {
            method: HttpMethod::Patch,
            path,
            parameters,
            body,
        }
    }

    pub fn delete(
        path: String,
        parameters: Option<Vec<(String, String)>>,
        body: Option<String>,
    ) -> Self {
        HttpRequest {
            method: HttpMethod::Delete,
            path,
            parameters,
            body,
        }
    }

    //TODO testing function
    pub fn test() {
        HttpRequest::new(HttpMethod::Get, "/".to_string(), Some(vec![]), None);
    }
}
