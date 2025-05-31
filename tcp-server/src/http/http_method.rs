/*
Copyright 2025 CS 462 Personal Data Acquisition Prototype Group
    
Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License.
You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
*/
//methods the server allows along with a catchall Error
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Delete,
    Options,
    Error,
}

impl HttpMethod {
    //const of all possible types to all iteration over all possible values
    pub const ALL_TYPES: [HttpMethod; 6] = [
        Self::Get,
        Self::Post,
        Self::Patch,
        Self::Delete,
        Self::Options,
        Self::Error,
    ];

    //Returns the String representitive
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
            Self::Options => "OPTIONS",
            Self::Error => "ERROR",
        }
    }

    #[allow(unused)]
    pub fn from_string(string: String) -> Self {
        for method in Self::ALL_TYPES {
            if string.starts_with(method.as_str()) {
                return method;
            }
        }
        HttpMethod::Error
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.as_str().to_string().into_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        for method in Self::ALL_TYPES {
            if bytes.starts_with(&method.to_bytes()) {
                return method;
            }
        }
        HttpMethod::Error
    }
}
