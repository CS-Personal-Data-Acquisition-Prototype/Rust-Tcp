/*
Copyright 2025 CS 462 Personal Data Acquisition Prototype Group
    
Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License.
You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
*/
use serde_json::Value;
use std::{fs, str};
use url::form_urlencoded;

use crate::HTTP_HEADER_DELIMITER;

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
            self.method.as_str(),
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
        let (header, _body) = buffer.split_at(
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
                headers.insert(k.to_lowercase(), v.to_string());
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

        HttpRequest {
            method,
            path: HttpPath::from_string(path),
            parameters,
            headers,
            body: None,
        }
    }

    pub fn parse_body(&mut self, buffer: &[u8]) -> crate::Result<()> {
        //trim delimiter and any extra whitespace
        let trim_body = String::from_utf8_lossy(buffer)
            .trim_end_matches('\0')
            .trim_end_matches(&String::from_utf8_lossy(HTTP_HEADER_DELIMITER).to_string())
            .to_string();

        if trim_body.len() == 0 {
            return Err("Failed to parse request body to utf8 string".to_string());
        }

        //parse buffer to Value object
        self.body = match serde_json::from_str::<Value>(&trim_body) {
            Ok(value) => Some(value),
            Err(e) => {
                #[cfg(debug_assertions)]
                let _ = fs::write("failed_parse.txt", trim_body);
                return Err(format!("Failed to parse request body to json value: {e}"));
            }
        };
        Ok(())
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
